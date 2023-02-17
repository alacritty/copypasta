# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.2

### Packaging

- Minimum rust version was bumped to `1.60.0`

### Fixed

- `x11_clipboard` was bumped to `0.7.0` droping `quick-xml` from the deps tree


## 0.8.1 

### Fixed

- Crash on use-after-free on macOS

## 0.8.0

### Packaging

- Minimum rust version was bumped to `1.57.0`

### Fixed

- Memory leak on macOS

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
