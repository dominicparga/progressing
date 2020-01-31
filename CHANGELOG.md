# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
1. [v2.1.0](#v2_1_0)
1. [v2.0.2](#v2_0_2)
1. [v2.0.1](#v2_0_1)
1. [v2.0.0](#v2_0_0)
1. [v1.0.3](#v1_0_3)
1. [v1.0.2](#v1_0_2)


## [Unreleased] <a name="unreleased"></a>

### Added

\-


### Changed

\-


### Deprecated

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>---]` -> `[====o---]`)


### Removed

\-


### Fixed

\-


### Security

\-



## [v2.1.0] <a name="v2_1_0"></a>

### Added

- `GitHub`-workflow auto-testing and -publishing to `crates.io` if `Cargo.toml` changes version and tests are successful.
- Add a little documentation to `struct`s with little examples from `simple`-example.
- Add support for `usize` for `MappingBar`.


### Changed

- Move `structs` in own `modules`.
- Change bar-style slightly: `[====>   ]` -> `[====>---]`


### Deprecated

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.2] <a name="v2_0_2"></a>

### Fixed

- `README.md` has had invalid code, which as been edited as in `examples`.

### Deprecated

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.1] <a name="v2_0_1"></a>

### Changed

- Changelog has been edited accordingly.


### Deprecated

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.0] <a name="v2_0_0"></a>

### Added

- Add access to a bar's `progress`.
- Extend `BernoulliProgress`
  - Implement some useful traits like `Add` and `Copy`.
  - Implement automatic `into()` for tuples (`(successes, attempts)`) or successes (`u32` or `bool`).
  - Implement constructing method `new()` and `Default`.


### Changed

- `ID`s in `CHANGELOG.md` since underscores (`_`) are preferred over dots (`.`) in `URL`s, but `v1103` could stand for `v11.0.3` and `v1.10.3`.


### Deprecated

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)
- Changelog has been forgotten to adapt..


## [v1.0.3] <a name="v1_0_3"></a>

### Added

- Implement `CHANGELOG.md`


### Deprecated

- `BernoulliProgress` has no constructing method like `new()` and no default-implementation.
- Detailled documentation is missing, though examples are good.


## [v1.0.2] <a name="v1_0_2"></a>

### Added

- Setup repo with license, readme, .gitignore etc.
  - Add nice `examples`.
- Implement 3 different `progressbars` and let them accept values outside of their intervals.
  - One is clamping `float`s to `[0, 1]`.
  - One is counting and mapping `i32` or `u32` from `[a, b]` to `[0, 1]`.
  - One is counting successes and attempts for a given goal `n`.
- Change a bar's progress via methods `set` and `add` using
  - `BernoulliProgress` (which is `pub`)- Support for `GitHub-workflow` is missing, which can, besides testing, automatically deploy to `crates.io` and tag if `Cargo.toml` changes version.
- `CHANGELOG.md` is missing
- `BernoulliProgress` has no constructing method like `new()` and no default-implementation.
- Detailled documentation is missing, though examples are good.


[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[Unreleased]: https://github.com/dominicparga/progressing/compare/v2.1.0...HEAD
[v2.1.0]: https://github.com/dominicparga/progressing/compare/v2.0.2...v2.1.0
[v2.0.2]: https://github.com/dominicparga/progressing/compare/v2.0.1...v2.0.2
[v2.0.1]: https://github.com/dominicparga/progressing/compare/v2.0.0...v2.0.1
[v2.0.0]: https://github.com/dominicparga/progressing/compare/v1.0.3...v2.0.0
[v1.0.3]: https://github.com/dominicparga/progressing/compare/v1.0.2...v1.0.3
[v1.0.2]: https://github.com/dominicparga/progressing/releases/tag/v1.0.2
