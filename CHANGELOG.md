# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1-alpha.3] - 2020-07-21

### Added

- Handle `login`/`logout`, saving the token to the system login keychain
- `wait` for a build by specifying:

  - `--url`, or
  - `--organization`, `--pipeline` and `--number`

- Trigger a system notification on build completion with `--notification`
- Output the content of the notification as a JSON object with `--output-notification-json`

[unreleased]: https://github.com/liamdawson/buildkite_waiter/compare/v0.0.1-alpha.3...HEAD
[0.0.1-alpha.3]: https://github.com/liamdawson/buildkite_waiter/releases/tag/v0.0.1-alpha.3
