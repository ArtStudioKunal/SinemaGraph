# Sinemagraph

Sinemagraph is a customized open-source non-linear video editor based on [FilmCraft](https://github.com/storytold/filmcraft). Its original project-file format and internal Rust library names are retained to avoid compatibility regressions.

The Sinemagraph logo is the project owner's supplied black-and-white clapperboard artwork.

**Build by Kunal Sagar for query contact artstudio.kunalsagar@gmail.com**

### Windows EXE

The workflow [Sinemagraph Windows EXE](../../actions/workflows/sinemagraph-windows.yml) builds the Rust desktop editor on Windows x64 and publishes `Sinemagraph-Windows-x64` as an Actions artifact. The workflow generates the Sinemagraph icons before compilation.

**Download existing tested build:** [Sinemagraph Windows x64 Actions run](https://github.com/ArtStudioKunal/filmcraft/actions/runs/43229950271), then open the `Sinemagraph-Windows-x64` artifact. The ZIP contains `sinemagraph.exe` and notices.

**Publish a GitHub preview release:** Open [Sinemagraph Windows EXE workflow](https://github.com/ArtStudioKunal/filmcraft/actions/workflows/sinemagraph-windows.yml), choose **Run workflow**, select `main` and click **Run workflow**. When the run succeeds, it publishes a pre-release with the EXE and a bundled ZIP to [Releases](https://github.com/ArtStudioKunal/filmcraft/releases). The preview is not a stable, Windows-GUI-tested final release.

For a local Windows build, install the FilmCraft Rust prerequisites and run:

```powershell
python -m pip install Pillow
python tools/sinemagraph/generate_icons.py
cargo build --locked --release -p filmcraft --bin sinemagraph
```

The output is `target/release/sinemagraph.exe`.

### Differences from upstream

- Sinemagraph desktop title and Windows EXE metadata
- Custom user-supplied logo for taskbar and application icons
- New sessions start empty unless explicitly asked to open a demo
- Startup software update checks disabled
- Top-level Help menu removed; About is last and displays the supplied contact text

**Licensing:** Original upstream license, attribution and notices remain intact. Do not redistribute the user-supplied logo without permission.
