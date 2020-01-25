# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#Unreleased)
1. [v1.0.3](#v1.0.3)
1. [v1.0.2](#v1.0.2)


## [Unreleased]

### Added

- Add access to a bar's `progress`.
- Extend `BernoulliProgress`
  - Implement some useful traits like `Add` and `Copy`.
  - Implement automatic `into()` for tuples (`(successes, attempts)`) or successes (`u32` or `bool`).
  - Implement constructing method `new()` and `Default`.


### Changed

\-


### Deprecated

- Detailled documentation is missing, though examples are good.


### Removed

\-


### Fixed

\-


### Security

\-


## [v1.0.3]

### Added

- Implement `CHANGELOG.md`

### Deprecated

- `BernoulliProgress` has no constructing method like `new()` and no default-implementation.
- Detailled documentation is missing, though examples are good.


## [v1.0.2]

### Added

- Setup repo with license, readme, .gitignore etc.
  - Add nice `examples`.
- Implement 3 different `progressbars` and let them accept values outside of their intervals.
  - One is clamping `float`s to `[0, 1]`.
  - One is counting and mapping `i32` or `u32` from `[a, b]` to `[0, 1]`.
  - One is counting successes and attempts for a given goal `n`.
- Change a bar's progress via methods `set` and `add` using
  - `BernoulliProgress` (which is `pub`)


### Deprecated

- `CHANGELOG.md` is missing
- `BernoulliProgress` has no constructing method like `new()` and no default-implementation.
- Detailled documentation is missing, though examples are good.


### Changed

### Removed

### Fixed

### Security


[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[Unreleased]: https://github.com/dominicparga/progressing/compare/v1.0.3...HEAD
[v1.0.3]: https://github.com/dominicparga/progressing/compare/v1.0.2...v1.0.3
[v1.0.2]: https://github.com/dominicparga/progressing/releases/tag/v1.0.2
