# Lydkontroll website

Isolated static marketing site for GitHub Pages. It does not share dependencies
or build output with the Tauri application.

```bash
npm ci
npm run lint
npm run build
```

The production build uses Vite's `/lydkontroll/` base path and writes to
`website/dist/`. Deployment is owned by `.github/workflows/pages.yml`.
