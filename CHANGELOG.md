# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Notes

- Simplified internal code structure (to a single crate).

## [0.2.1] - 2020-08-14

### Changed

- Progress polling will now retry (up to 3 times) on failure

## [0.2.0] - 2020-07-29

### Changed

- OS notification support is now gated behind a default feature, `os-notifications`
- OS notifications will be shown by default

  - `--notification` is now deprecated

### Added

- `--no-notification` to prevent OS notifications from being sent

### Fixed

- Improved reliability of OS notifications on Windows

## [0.1.1] - 2020-07-24

### Added

- CLI completions

## [0.1.0] - 2020-07-24

### Changed

- `wait --url` is now `by-url`
- `wait --organization --pipeline --number` is now `by-number`

### Removed

- `--output-notification-json`

### Added

- `latest` to wait for the latest build matching the options

  - Includes support for `--branch`, `--state`, `--creator` and `--commit` options
  - Supports `--mine` to filter by builds created by the API Access Token owner

- New `--output` formats:

  - `none`, which outputs nothing to stdout
  - `state-url` (default), which outputs the build state and web console URL
  - `notification-lines`, which outputs the title and message of the system notification on separate lines

## [0.0.1-alpha.3] - 2020-07-21

### Added

- Handle `login`/`logout`, saving the token to the system login keychain
- `wait` for a build by specifying:

  - `--url`, or
  - `--organization`, `--pipeline` and `--number`

- Trigger a system notification on build completion with `--notification`
- Output the content of the notification as a JSON object with `--output-notification-json`

[unreleased]: https://github.com/liamdawson/buildkite_waiter/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/liamdawson/buildkite_waiter/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/liamdawson/buildkite_waiter/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/liamdawson/buildkite_waiter/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/liamdawson/buildkite_waiter/compare/v0.0.1-alpha.3...v0.1.0
[0.0.1-alpha.3]: https://github.com/liamdawson/buildkite_waiter/releases/tag/v0.0.1-alpha.3
