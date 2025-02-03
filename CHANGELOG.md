# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-02-03

### Changed
- Replaced Axum with Loco framework in web interface
- Added MVC structure for web project
- Implemented basic chat controller with REST API endpoint
- Added configuration system with YAML support

## [0.2.0] - 2025-02-03

### Added
- Restructured project as a Rust workspace with three main crates:
  - `domestic-cli`: Command-line interface for Ollama integration
  - `domestic-web`: Web interface with Axum framework and health check endpoint
  - `domestic-management`: Management utilities for models and system configuration
- Set up workspace-level dependencies management
- Added shared dependencies: tokio, serde, tracing
- Implemented basic web server with health check endpoint
- Added logging and tracing support across all crates
- Configured cross-crate version management

### Changed
- Moved original Ollama integration code to CLI crate
- Updated project structure to support multiple interfaces
- Enhanced build system with workspace-wide configuration

## [0.1.0] - 2025-02-03

### Added
- Initial commit
- Project foundation
- Basic documentation structure
