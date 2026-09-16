# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.5.0] - 2026-09-16

### Changed
- Simplified Dockerfile by removing multi-architecture support
- Removed ARM64 and ARMv7 cross-compilation stages
- Reduced Dockerfile from ~71 lines to ~25 lines for better maintainability
- Build process now targets only AMD64/x86_64 architecture

### Fixed
- Fixed shell syntax errors in Dockerfile (string comparison operators)
- Fixed Clippy warning: replaced `.len() == 0` with `.is_empty()` in metrics.rs
- Fixed Clippy warning: removed empty line after outer attribute in client.rs
- Removed unnecessary build arguments and debug statements from Dockerfile

### Removed
- ARM64 (aarch64) architecture support
- ARMv7 architecture support
- ARM-specific dependencies (gcc-aarch64-linux-gnu, gcc-arm-linux-gnueabihf, patchelf)
- Multi-stage builder configurations for ARM platforms

## [2.1.2] - Previous Release

### Added
- Multi-zone support for tado° smart thermostats
- Weather monitoring (outside temperature and solar intensity)
- Window detection per zone
- Dual temperature units (Celsius and Fahrenheit)
- AC & Heating power metrics
- Docker support with multi-registry push (Docker Hub + GHCR)
- Grafana dashboard integration (#13847)
- OAuth2 Device Code Grant Flow authentication
- Automated token refresh management
- CI/CD with GitHub Actions (tests, clippy, format checks)
- Dependabot for automated dependency updates
- Weekly Cargo.lock updates

[2.5.0]: https://github.com/IamTheLoki/tado-exporter/compare/v2.1.2...v2.5.0
[2.1.2]: https://github.com/IamTheLoki/tado-exporter/releases/tag/v2.1.2
