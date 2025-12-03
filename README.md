# N-Dimensional Projection Viewer

## Features
- **Shape switching** between N-cubes, N-simplices, and N-orthoplexes (cross polytopes) so you can explore dual families.
- **Manual plane control**: iterate through rotation planes and hold the arrow keys to rotate in real-time.
- **Dynamic scaling** tuned per shape so higher-dimensional simplexes stay visible alongside cubes.
- **HUD overlay** that surfaces the current shape, dimension, vertex count, plane selection, and control cheatsheet.
- **WebGL renderer** that draws edges/vertices via GPU vertex buffers for a ~20× faster draw stage, with Canvas 2D as a graceful fallback.
- **WebAssembly acceleration**: a Rust core (via `wasm-pack`) handles the rotation/projection pipeline with automatic JS fallback when the module is unavailable.
- **Responsive UI & touch controls**: the canvas/HUD scale down for phones and tablets, every keyboard shortcut is mirrored by on-screen buttons, and dedicated on-canvas pads handle zooming and panning when you do not have a keyboard or mouse.

## Controls
| Action | Keys |
| --- | --- |
| Next/previous shape | `TAB` / `Shift + TAB` or on-canvas Shape buttons |
| Increase/decrease dimension | `=` / `-` or on-canvas `N +/-` |
| Cycle active rotation plane | `↑` / `↓` or Plane buttons |
| Rotate active plane | hold `←` / `→` or Rotate buttons |
| Reset active plane angle | `/` |
| Reset all angles | `R` |
| Restore defaults | `Esc` |
| Pan viewport | `W`/`A`/`S`/`D` or on-canvas pad |
| Zoom viewport | Mouse wheel (canvas focused) or +/- on-canvas buttons |

> On touch devices, use the on-screen buttons (Shape </>, N +/-, Plane </>, Rotate </>, Zero, Reset, Defaults) beneath the canvas plus the left pad (pan) and right +/- buttons (zoom) that sit on top of the canvas to trigger the same actions without a keyboard.

When the canvas has keyboard focus, the mouse wheel zooms smoothly toward the canvas center and WASD pans at a constant screen-space speed, matching the feel of the touch buttons.

## Running Locally
1. **Build the WebAssembly module (optional but recommended):** Install [Rust](https://www.rust-lang.org/tools/install) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), then run:
    ```bash
    cd projection/wasm
    wasm-pack build --release --target web --out-dir ../pkg
    ```
    This produces `pkg/` with the `.wasm` binary plus JS glue that `index.html` loads. If you skip this step, the app automatically falls back to the (slower) JavaScript implementation.
2. Open `projection/index.html` directly in any modern browser, or serve the folder via a lightweight HTTP server (e.g. `python -m http.server`).
3. Interact with the canvas using the controls listed above.

## GitHub Pages Deployment
This repository already includes a GitHub Actions workflow (`.github/workflows/pages.yml`) that publishes the static site to GitHub Pages.

1. Push the repository to GitHub and ensure your default branch is `main` (update the workflow trigger if you use a different branch).
2. In the GitHub repository, go to **Settings → Pages** and set the source to **GitHub Actions**.
3. On the next push to `main`, the workflow will run `wasm-pack build`, bundle the generated `pkg/`, and deploy everything in the repository root (including `index.html`) to GitHub Pages automatically.

You can also run the workflow manually via the **Actions** tab → **Deploy static site** → **Run workflow** to force a redeploy.

## Project Structure
```
projection/
├── index.html        # WebGL app with embedded styles/scripts (loads WASM if available)
├── README.md         # This documentation
├── wasm/             # Rust crate compiled to WebAssembly
│   └── src/lib.rs
├── pkg/              # wasm-pack output (generated)
└── .github/
    └── workflows/
        └── pages.yml  # GitHub Pages deploy pipeline
```

Feel free to customise the styles, extend with additional shapes, or integrate with other hosting setups. Contributions welcome!

