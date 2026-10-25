# Project Rules for tado-exporter

## Automatic File Updates on Changes
- **Cargo.toml / Versioning:** On code changes, new features or bug fixes, always update the version number in `Cargo.toml` according to the SemVer convention.
- **Versioning:** Follow the SemVer convention for new releases:
  - **MAJOR:** Breaking changes or incompatible API changes
  - **MINOR:** New features (backwards compatible)
  - **PATCH:** Bug fixes and minor adjustments (backwards compatible)
- **CHANGELOG.md:** On every functional change or release, automatically update `CHANGELOG.md`. Add new features under `[Unreleased]` or the new version number.
- **Badges:** Keep the badges directly below the README title (Docker Pulls, Docker Hub, GHCR, Rust, Grafana) up to date, e.g. on a new Rust edition (see `Cargo.toml`), changed image names or a new Grafana dashboard ID
- **Code style:** Strictly follow idiomatic Rust (Rust 2024 edition).
- **Notification:** Explicitly inform the user about all automatic updates to Cargo.toml, CHANGELOG.md and other files according to these rules.
