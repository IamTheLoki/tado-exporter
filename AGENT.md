# Projekt-Regeln für tado-exporter

## Automatische Datei-Updates bei Änderungen
- **Cargo.toml / Versioning:** Wenn Code-Änderungen, neue Features oder Bugfixes umgesetzt werden, passe immer die Versionsnummer in der `Cargo.toml` entsprechend der SemVer-Konvention an.
- **Versionierung:** Folge der SemVer-Konvention für neue Releases:
  - **MAJOR:** Breaking Changes oder incompatible API-Änderungen
  - **MINOR:** Neue Features (abwärtskompatibel)
  - **PATCH:** Bugfixes und kleinere Anpassungen (abwärtskompatibel)
- **CHANGELOG.md:** Aktualisiere bei jeder funktionellen Änderung oder bei einem Release automatisch die `CHANGELOG.md`. Füge neue Features unter `[Unreleased]` oder der neuen Versionsnummer hinzu.
- **Code-Stil:** Halte dich strikt an idomatisches Rust (Rust 2024 Edition).
- **Benachrichtigung:** Informiere den Benutzer explizit über alle automatischen Updates an Cargo.toml, CHANGELOG.md und anderen Dateien gemäß dieser Regeln. 
