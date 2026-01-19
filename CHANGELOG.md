# Changelog

All notable changes to RururuOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Core System**
  - Universal file handler with codec detection
  - Plugin architecture for file type extensions
  - Thumbnail generation system with caching

- **Desktop Applications**
  - `rururu-files` - Modern file manager with tags and batch operations
  - `rururu-settings` - System settings application
  - `rururu-monitor` - System monitor with process management
  - `rururu-colorcal` - Display calibration tool

- **Color Management**
  - `rururu-color` - Color management daemon
  - ICC profile management
  - OpenColorIO integration
  - HDR display support
  - D-Bus interface for system-wide color management

- **Workflow Profiles**
  - `rururu-workflows` - Workflow profile management
  - Video Editor profile (DaVinci Resolve, Kdenlive)
  - 3D Artist profile (Blender, FreeCAD)
  - 2D Designer profile (Krita, GIMP, Inkscape)
  - Audio Producer profile (Ardour, Bitwig)
  - Photographer profile (Darktable, RawTherapee)
  - CLI tool for workflow switching

- **Desktop Environment Configurations**
  - Sway configuration with creative workflow optimizations
  - Tokyo Night and Catppuccin themes
  - Creative workspace scripts
  - GNOME GSchema for RururuOS settings
  - KDE Creative Launcher plasmoid

- **Installer**
  - Calamares configuration
  - RururuOS branding
  - Workflow selection module
  - Hardware auto-detection
  - Post-install setup wizard

- **ISO Build System**
  - x86_64 ISO build scripts
  - ARM64 Raspberry Pi image builder
  - ARM64 Apple Silicon (Asahi) overlay
  - Makefile for build automation

- **CI/CD**
  - GitHub Actions for CI
  - Automated release builds
  - Nightly ISO builds
  - ARM64 cross-compilation

- **Documentation**
  - Installation guide
  - User manual
  - Developer guide
  - Troubleshooting guide
  - FAQ

### Changed

- N/A (initial release)

### Deprecated

- N/A (initial release)

### Removed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Security

- N/A (initial release)

## [1.0.0] - TBD

Initial stable release.

---

## Version History

| Version | Date | Type |
|---------|------|------|
| 1.0.0 | TBD | Stable |
| 1.0.0-rc.1 | TBD | Release Candidate |
| 1.0.0-beta.1 | TBD | Beta |
| 1.0.0-alpha.1 | TBD | Alpha |
