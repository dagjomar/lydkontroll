use std::{
    collections::HashSet,
    io,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use thiserror::Error;

pub const DEFAULT_TAILSCALE_TIMEOUT: Duration = Duration::from_secs(3);
const TAILSCALE_APP_PATH: &str = "/Applications/Tailscale.app/Contents/MacOS/Tailscale";
const TAILSCALE_LOCAL_PATH: &str = "/usr/local/bin/tailscale";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait ProcessRunner {
    fn run(
        &self,
        executable: &Path,
        arguments: &[&str],
        timeout: Duration,
    ) -> Result<CommandOutput, ProcessRunError>;
}

#[derive(Debug, Error)]
pub enum ProcessRunError {
    #[error("executable not found")]
    NotFound,
    #[error("command timed out after {timeout:?}")]
    Timeout { timeout: Duration },
    #[error("could not run command: {0}")]
    Io(#[source] io::Error),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(
        &self,
        executable: &Path,
        arguments: &[&str],
        timeout: Duration,
    ) -> Result<CommandOutput, ProcessRunError> {
        let mut child = Command::new(executable)
            .args(arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| match error.kind() {
                io::ErrorKind::NotFound => ProcessRunError::NotFound,
                _ => ProcessRunError::Io(error),
            })?;
        let deadline = Instant::now() + timeout;
        loop {
            match child.try_wait().map_err(ProcessRunError::Io)? {
                Some(status) => {
                    let output = child.wait_with_output().map_err(ProcessRunError::Io)?;
                    return Ok(CommandOutput {
                        success: status.success(),
                        stdout: output.stdout,
                        stderr: output.stderr,
                    });
                }
                None if Instant::now() >= deadline => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(ProcessRunError::Timeout { timeout });
                }
                None => thread::sleep(Duration::from_millis(10)),
            }
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TailscaleDiscoveryError {
    #[error("Tailscale CLI was not found in any configured location")]
    CliNotFound,
    #[error("Tailscale CLI timed out after {timeout:?}")]
    Timeout { timeout: Duration },
    #[error("Tailscale CLI failed: {message}")]
    CommandFailed { message: String },
    #[error("Tailscale CLI output was not valid UTF-8")]
    InvalidUtf8,
    #[error("Tailscale CLI returned no IPv4 address")]
    NoAddress,
    #[error("Tailscale CLI returned more than one IPv4 address")]
    MultipleAddresses,
    #[error("Tailscale CLI returned an invalid IPv4 address: {value}")]
    InvalidAddress { value: String },
    #[error("address {address} is outside the Tailscale IPv4 range")]
    OutsideTailscaleRange { address: Ipv4Addr },
    #[error("address {address} is not assigned to this Mac")]
    AddressNotLocal { address: Ipv4Addr },
    #[error("could not inspect local network addresses: {message}")]
    LocalAddressInspection { message: String },
}

pub fn discover_tailscale_ipv4(
    runner: &impl ProcessRunner,
    configured_executable: Option<&Path>,
    local_addresses: &HashSet<Ipv4Addr>,
    timeout: Duration,
) -> Result<Ipv4Addr, TailscaleDiscoveryError> {
    let mut candidates = Vec::new();
    if let Some(path) = configured_executable {
        candidates.push(path.to_path_buf());
    }
    for candidate in [
        PathBuf::from("tailscale"),
        PathBuf::from(TAILSCALE_APP_PATH),
        PathBuf::from(TAILSCALE_LOCAL_PATH),
    ] {
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }

    let mut last_failure = None;
    for candidate in candidates {
        match runner.run(&candidate, &["ip", "-4"], timeout) {
            Ok(output) if output.success => {
                return parse_and_validate(&output.stdout, local_addresses);
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
                last_failure = Some(TailscaleDiscoveryError::CommandFailed {
                    message: if stderr.is_empty() {
                        format!("{} exited unsuccessfully", candidate.display())
                    } else {
                        stderr
                    },
                });
            }
            Err(ProcessRunError::NotFound) => {}
            Err(ProcessRunError::Timeout { timeout }) => {
                return Err(TailscaleDiscoveryError::Timeout { timeout });
            }
            Err(ProcessRunError::Io(error)) => {
                last_failure = Some(TailscaleDiscoveryError::CommandFailed {
                    message: format!("{}: {error}", candidate.display()),
                });
            }
        }
    }
    Err(last_failure.unwrap_or(TailscaleDiscoveryError::CliNotFound))
}

pub fn local_ipv4_addresses() -> Result<HashSet<Ipv4Addr>, TailscaleDiscoveryError> {
    if_addrs::get_if_addrs()
        .map_err(|error| TailscaleDiscoveryError::LocalAddressInspection {
            message: error.to_string(),
        })
        .map(|interfaces| {
            interfaces
                .into_iter()
                .filter_map(|interface| match interface.addr {
                    if_addrs::IfAddr::V4(address) => Some(address.ip),
                    if_addrs::IfAddr::V6(_) => None,
                })
                .collect()
        })
}

fn parse_and_validate(
    stdout: &[u8],
    local_addresses: &HashSet<Ipv4Addr>,
) -> Result<Ipv4Addr, TailscaleDiscoveryError> {
    let output = std::str::from_utf8(stdout).map_err(|_| TailscaleDiscoveryError::InvalidUtf8)?;
    let values = output.split_whitespace().collect::<Vec<_>>();
    let value = match values.as_slice() {
        [] => return Err(TailscaleDiscoveryError::NoAddress),
        [value] => *value,
        _ => return Err(TailscaleDiscoveryError::MultipleAddresses),
    };
    let address =
        value
            .parse::<Ipv4Addr>()
            .map_err(|_| TailscaleDiscoveryError::InvalidAddress {
                value: value.to_owned(),
            })?;
    if !is_tailscale_ipv4(address) {
        return Err(TailscaleDiscoveryError::OutsideTailscaleRange { address });
    }
    if !local_addresses.contains(&address) {
        return Err(TailscaleDiscoveryError::AddressNotLocal { address });
    }
    Ok(address)
}

fn is_tailscale_ipv4(address: Ipv4Addr) -> bool {
    let octets = address.octets();
    octets[0] == 100 && (64..=127).contains(&octets[1])
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, sync::Mutex};

    use super::*;

    #[derive(Debug)]
    struct FakeRunner {
        results: Mutex<VecDeque<Result<CommandOutput, ProcessRunError>>>,
        executables: Mutex<Vec<PathBuf>>,
    }

    impl FakeRunner {
        fn new(results: Vec<Result<CommandOutput, ProcessRunError>>) -> Self {
            Self {
                results: Mutex::new(results.into()),
                executables: Mutex::new(Vec::new()),
            }
        }
    }

    impl ProcessRunner for FakeRunner {
        fn run(
            &self,
            executable: &Path,
            _arguments: &[&str],
            _timeout: Duration,
        ) -> Result<CommandOutput, ProcessRunError> {
            self.executables
                .lock()
                .expect("executables")
                .push(executable.to_path_buf());
            self.results
                .lock()
                .expect("results")
                .pop_front()
                .expect("configured fake result")
        }
    }

    fn success(stdout: &str) -> Result<CommandOutput, ProcessRunError> {
        Ok(CommandOutput {
            success: true,
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        })
    }

    #[test]
    fn configured_candidate_wins_and_validates_local_tailscale_address() {
        let address = Ipv4Addr::new(100, 64, 1, 2);
        let runner = FakeRunner::new(vec![success("100.64.1.2\n")]);

        let result = discover_tailscale_ipv4(
            &runner,
            Some(Path::new("/custom/tailscale")),
            &HashSet::from([address]),
            DEFAULT_TAILSCALE_TIMEOUT,
        );

        assert_eq!(result, Ok(address));
        assert_eq!(
            runner.executables.lock().expect("executables").as_slice(),
            [PathBuf::from("/custom/tailscale")]
        );
    }

    #[test]
    fn missing_candidates_fall_through_to_packaged_application_path() {
        let address = Ipv4Addr::new(100, 100, 2, 3);
        let runner = FakeRunner::new(vec![Err(ProcessRunError::NotFound), success("100.100.2.3")]);

        assert_eq!(
            discover_tailscale_ipv4(
                &runner,
                None,
                &HashSet::from([address]),
                DEFAULT_TAILSCALE_TIMEOUT,
            ),
            Ok(address)
        );
        assert_eq!(
            runner.executables.lock().expect("executables").as_slice(),
            [
                PathBuf::from("tailscale"),
                PathBuf::from(TAILSCALE_APP_PATH)
            ]
        );
    }

    #[test]
    fn discovery_rejects_multiple_non_tailscale_and_non_local_addresses() {
        let local = HashSet::from([Ipv4Addr::new(100, 64, 1, 2)]);
        assert_eq!(
            parse_and_validate(b"100.64.1.2\n100.64.1.3\n", &local),
            Err(TailscaleDiscoveryError::MultipleAddresses)
        );
        assert_eq!(
            parse_and_validate(
                b"192.168.1.2\n",
                &HashSet::from([Ipv4Addr::new(192, 168, 1, 2)])
            ),
            Err(TailscaleDiscoveryError::OutsideTailscaleRange {
                address: Ipv4Addr::new(192, 168, 1, 2)
            })
        );
        assert_eq!(
            parse_and_validate(b"100.64.1.3\n", &local),
            Err(TailscaleDiscoveryError::AddressNotLocal {
                address: Ipv4Addr::new(100, 64, 1, 3)
            })
        );
    }

    #[test]
    fn timeout_and_invalid_output_are_typed() {
        let runner = FakeRunner::new(vec![Err(ProcessRunError::Timeout {
            timeout: DEFAULT_TAILSCALE_TIMEOUT,
        })]);
        assert_eq!(
            discover_tailscale_ipv4(&runner, None, &HashSet::new(), DEFAULT_TAILSCALE_TIMEOUT,),
            Err(TailscaleDiscoveryError::Timeout {
                timeout: DEFAULT_TAILSCALE_TIMEOUT
            })
        );
        assert_eq!(
            parse_and_validate(b"not-an-ip", &HashSet::new()),
            Err(TailscaleDiscoveryError::InvalidAddress {
                value: "not-an-ip".to_owned()
            })
        );
        assert_eq!(
            parse_and_validate(b"", &HashSet::new()),
            Err(TailscaleDiscoveryError::NoAddress)
        );
    }
}
