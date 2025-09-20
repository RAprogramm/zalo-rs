# Changelog

All notable changes to this project will be documented in this file.

## [0.1.3] - 2024-05-23

### Added
- Support for overriding the configuration file path via the `ZALO_BOT_CONFIG_PATH` environment variable in `ConfigLoader`, including error propagation when the file is missing.

### Changed
- The `bot-axum` example test now expects a configuration error when the override points to a missing file, matching the loader behaviour.

## [0.1.2] - 2024-05-22

### Added
- Russian translation stored in `docs/README.ru.md` with cross-links to English and Vietnamese guides.

### Changed
- Highlighted all three supported languages in the primary README and Vietnamese translation index.


## [0.1.1] - 2024-05-21

### Added
- English primary README with structured platform overview and workflow guidance.
- Vietnamese translation stored in `docs/README.vi.md` with cross-links between languages.
- Minimal `deny.toml` to share advisory database settings for `cargo deny`.

### Changed
- Highlighted security and dependency checks (`cargo audit`, `cargo deny`) in the quality gate section.

## [0.1.0] - 2024-05-21

### Added
- Initial workspace layout, documentation, and examples for Zalo OA integration in Rust.

