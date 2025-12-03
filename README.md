# N-Dimensional Projection Viewer

Interactive orthographic projection of high-dimensional cubes and simplices, rewritten for the web. The viewer renders polytopes onto a 2D canvas with per-plane manual rotations and HUD guidance so you can explore geometric relationships across dimensions right in your browser.

## Features
- **Shape switching** between N-cubes and N-simplices (regular tetrahedra generalised to N+1 equidistant vertices).
- **Manual plane control**: iterate through rotation planes and hold the arrow keys to rotate in real-time.
- **Dynamic scaling** tuned per shape so higher-dimensional simplexes stay visible alongside cubes.
- **HUD overlay** that surfaces the current shape, dimension, vertex count, plane selection, and control cheatsheet.
- **Zero build tooling**: just HTML + CSS + vanilla JavaScript.

## Controls
| Action | Keys |
| --- | --- |
| Next/previous shape | `TAB` / `Shift + TAB` |
| Increase/decrease dimension | `=` / `-` |
| Cycle active rotation plane | `↑` / `↓` |
| Rotate active plane | hold `←` / `→` |
| Reset active plane angle | `/` |
| Reset all angles | `R` |
| Restore defaults | `Esc` |

## Running Locally
1. Open `index.html` directly in any modern browser, or serve the folder via a lightweight HTTP server (e.g. `python -m http.server`).
2. Interact with the canvas using the controls listed above.

## GitHub Pages Deployment
This repository already includes a GitHub Actions workflow (`.github/workflows/pages.yml`) that publishes the static site to GitHub Pages.

1. Push the repository to GitHub and ensure your default branch is `main` (update the workflow trigger if you use a different branch).
2. In the GitHub repository, go to **Settings → Pages** and set the source to **GitHub Actions**.
3. On the next push to `main`, the workflow will build and deploy the contents of the repo root (including `index.html`) to GitHub Pages automatically.

You can also run the workflow manually via the **Actions** tab → **Deploy static site** → **Run workflow** to force a redeploy.

## Project Structure
```
projection/
├── index.html      # Canvas app with embedded styles and scripts
├── README.md       # This documentation
└── .github/
    └── workflows/
        └── pages.yml  # GitHub Pages deploy pipeline
```

Feel free to customise the styles, extend with additional shapes, or integrate with other hosting setups. Contributions welcome!
