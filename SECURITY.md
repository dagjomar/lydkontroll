# Security policy

## Supported version

Only the latest commit on the default branch is currently supported. There are
no public binary releases. Source users are responsible for building from a
reviewed commit and keeping dependencies current.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Use the repository's
private GitHub vulnerability-reporting flow when available, or contact the
repository owner privately through their GitHub profile. Include affected
versions, reproduction steps, impact, and any suggested mitigation.

You should receive an acknowledgement within seven days. No guaranteed fix or
disclosure timeline is offered, but reports will be assessed before details are
made public.

## Security boundary

- Desktop playback and editing are local and require no network.
- Version-one mobile control requires Tailscale on the Mac and iPhone.
- The control server binds only to a validated local Tailscale IPv4 address on
  port `17321`; it must not fall back to loopback, LAN, or wildcard binding.
- Tailscale membership is the access-control boundary. Lydkontroll adds no
  password, PIN, or independent user authentication.
- Imported audio stays in application-managed storage on the Mac and is not
  sent to mobile clients.

Anyone with access to the relevant tailnet may be able to reach the controller.
Operators must secure their Tailscale accounts/devices and should not expose the
server through port forwarding or an ordinary public network.
