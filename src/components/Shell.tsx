import { useState } from "react";
import type { AppMode } from "../generated/AppMode";

const appMode: AppMode = "desktop";

export function Shell() {
  const [showStatus, setShowStatus] = useState(false);

  return (
    <main className="shell">
      <section className="hero" aria-labelledby="app-title">
        <p className="eyebrow">Marius + Wenche</p>
        <h1 id="app-title">Lydkontroll</h1>
        <p className="lede">
          Grunnskallet er klart. Cues, lydmotor og mobilkontroll kommer i de
          neste vertikale leveransene.
        </p>

        <button
          className="status-button"
          type="button"
          onClick={() => setShowStatus((visible) => !visible)}
          aria-expanded={showStatus}
        >
          {showStatus ? "Skjul systemstatus" : "Vis systemstatus"}
        </button>

        {showStatus ? (
          <div className="status-card" role="status">
            <span className="status-dot" aria-hidden="true" />
            <div>
              <strong>Appskall aktivt</strong>
              <p>
                Modus: {appMode}. Avspilling og nettverk er bevisst ikke koblet
                til ennå.
              </p>
            </div>
          </div>
        ) : null}
      </section>
    </main>
  );
}
