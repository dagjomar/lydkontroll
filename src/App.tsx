import { MobileControl } from "./components/MobileControl";
import { Shell } from "./components/Shell";
import { desktopApi } from "./services/desktopApi";

export function App() {
  return desktopApi.mode === "mobile" ? (
    <MobileControl api={desktopApi} />
  ) : (
    <Shell api={desktopApi} />
  );
}
