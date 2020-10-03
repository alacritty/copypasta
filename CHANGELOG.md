# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.1

### Changed

- Updated `smithay-clipboard` to 0.6.0

## 0.7.0

### Packaging

- Minimum rust version was bumped to `1.41.0`

### Removed

- Ability to create a Wayland clipboard from Display type directly using `create_clipboard`

## 0.6.3

### Added

- Features `x11` and `wayland` for picking the linux backends

## 0.6.2

### Fixed

- Compilation on iOS, using the no-op clipboard
