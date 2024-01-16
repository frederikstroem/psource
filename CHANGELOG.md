# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add merge CI/CD to verify Git tags, Cargo.toml version, CHANGELOG.md version and PKGBUILD version
- Add printing of pathing relative to home directory (if applicable)
- Add printing of pathing relative to root directory
- Include only files matching a regex
- Exclude files matching a regex
- Add tests
- Add `cargo test` CI/CD
- Make async
- Ability to omit comments in output
- Add option to print as markdown fenced code blocks, e.g.:

    ```markdown
        **src/main.rs:**
        ```rust
        fn main() {
            println!("Hello, world!");
        }
        ```
    ```
- Add video/gif demo to README.md
- Improve code quality and structure

## [v0.3.1] - 2024-01-16

### Fixed
- Fixed `psource --help` output in `README.md`.

## [v0.3.0] - 2024-01-16

### Added
- Git ancestry feature to display the file's ancestry within a Git repository.
- Added Dependency on `git2` crate to support Git.
- Added "Improve code quality and structure" to the roadmap in `CHANGELOG.md`.

### Changed
- Updated `clap` dependency.

### Removed
- Removed TODO items from the Unreleased section that have been implemented in this release.

## [v0.2.0] - 2024-01-01

### Added
- New CLI option `--ancestry <ANCESTRY>` to display the file's ancestry in the output path by including the specified number of parent directories relative to the current working directory, or 0 to omit the ancestry.

### Changed
- Updated the help message and usage instructions in the `README.md` to reflect the new `--ancestry` option.

### Removed
- Simplified the verification logic in `Config::new` by removing the redundant `verify_cli` function since clap already handles the conflict between `--stdout` and `--copy`.

### Fixed
- Conflict checks between `--stdout` and `--copy` flags in the CLI arguments.

## [v0.1.7] - 2023-12-28

### Fixed

- Fixed AUR publish workflow.

## [v0.1.6] - 2023-12-28

### Fixed

- Fixed AUR publish workflow.

## [v0.1.5] - 2023-12-28

### Fixed

- Missing entry in `CHANGELOG.md` for `v0.1.4`.
- Fixed AUR publish workflow.

## [v0.1.4] - 2023-12-28

### Added

- Publish package to the AUR.
- Add CI/CD to publish package to the AUR.
- Added AUR install instructions to `README.md`.

### Changed

- Updated badges in `README.md`.
- Rename GitHub workflow from `publish.yml` to `cargo_publish.yml`.

## [v0.1.3] - 2023-12-26

### Fixed

- Fix publish to crates.io workflow.

## [v0.1.2] - 2023-12-26

### Added

- Add CI/CD to publish to crates.io
- Cache to build and test CI/CD workflow.

### Changed

- Updated roadmap in `README.md`.

### Fixed

- Missing entry in `CHANGELOG.md` for `v0.1.1`.
- Changelog formatting.

## [v0.1.1] - 2023-12-26

### Added

- Shields for version, downloads, license, build status, and docs to `README.md`.

### Changed

- Renamed GitHub workflow file from `rust.yml` to `build_and_test.yml` and updated its name from "Rust" to "Build and test".
- Updated roadmap in `README.md`.
- Updated package version in `Cargo.toml` to `0.1.1`.

## [v0.1.0] - 2023-12-24

### Added

- Initial release of `psource`.
- Support for pretty printing source code to stdout.
- Ability to copy source code directly to the clipboard.
- Configuration file support with a setting to control the output target.
- Command-line interface (CLI) for easy interaction with the tool.
- Proper handling of binary files to skip them during the print process.
- Basic file walking to process directories and multiple files.
- Setup of a basic CI/CD pipeline for `cargo build`.

## [TEMPLATE] - YYYY-MM-DD

### Added

- N/A

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A
