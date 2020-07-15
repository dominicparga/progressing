# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
1. [v2.2.3](#v2.2.3)
    1. [v2.2.2](#v2.2.2)
    1. [v2.2.1](#v2.2.1)
    1. [v2.2.0](#v2.2.0)
    1. [v2.1.0](#v2.1.0)
    1. [v2.0.2](#v2.0.2)
        1. [v2.0.1](#v2.0.1)
        1. [v2.0.0](#v2.0.0)
1. [v1.0.3](#v1.0.3)
    1. [v1.0.2](#v1.0.2)


## [Unreleased][github/self/unreleased] <a name="unreleased"></a>

### Added <a name="unreleased/added"></a>

- Add `rustfmt` to github-workflows.
- Add tests running examples.
- Add wrapping `TimedBar` for approximating remaining time.
- Add printing bar dependent on relative progress (`has_progressed_much()` + `remember_progress()`).


### Changed <a name="unreleased/changed"></a>

- Update README.
- Make implementation much more explicit and easier by removing `reprint(...)` and refactoring `Bar`:
  - Change data-types to more convenient ones (e.g. u32 -> usize).
  - Implement `add(...)` based on `set(...)`
  - Implement other calculations for progress to support a generic progress-calculation.
    They are implemented only for some types, e.g. `usize`, `u64`, `f64`, `i64`, but new ones can be added easily.
  - Implement style using a `String` (e.g. `[=>-]` -> `(#..)`)


### Deprecated <a name="unreleased/deprecated"></a>

- Detailled documentation is missing, though examples are good.


### Removed <a name="unreleased/removed"></a>

- Remove `reprint(...)`-functions since they are dependent on the used writer/channel.
  -> Update examples.


### Fixed <a name="unreleased/fixed"></a>

\-


### Security <a name="unreleased/security"></a>

\-


## [v2.2.3][github/self/v2.2.3] <a name="v2.2.3"></a>

### Deprecated <a name="v2.2.3/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>---]` -> `[====o---]`)


### Fixed <a name="v2.2.3/fixed"></a>

- Fix __badge-link__ for license in `README.md`.


## [v2.2.2][github/self/v2.2.2] <a name="v2.2.2"></a>

### Changed <a name="v2.2.2/changed"></a>

- Change style of __LICENSE__ and write it with `markdown`.
- Replace branch `master` by `nightly`.


### Deprecated <a name="v2.2.2/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>---]` -> `[====o---]`)


## [v2.2.1][github/self/v2.2.1] <a name="v2.2.1"></a>

### Deprecated <a name="v2.2.1/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>---]` -> `[====o---]`)


### Fixed <a name="v2.2.1/fixed"></a>

- Fix URL in README.


## [v2.2.0][github/self/v2.2.0] <a name="v2.2.0"></a>

### Added <a name="v2.2.0/added"></a>

- Add `start(...)` and `end(...)` for `BernoulliBar` according to `MappingBar`.


### Deprecated <a name="v2.2.0/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>---]` -> `[====o---]`)
- README has wrong URL.


## [v2.1.0][github/self/v2.1.0] <a name="v2.1.0"></a>

### Added <a name="v2.1.0/added"></a>

- `GitHub`-workflow auto-testing and -publishing to `crates.io` if `Cargo.toml` changes version and tests are successful.
- Add a little documentation to `struct`s with little examples from `simple`-example.
- Add support for `usize` for `MappingBar`.


### Changed <a name="v2.1.0/changed"></a>

- Move `structs` in own `modules`.
- Change bar-style slightly: `[====>   ]` -> `[====>---]`


### Deprecated <a name="v2.1.0/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.2][github/self/v2.0.2] <a name="v2.0.2"></a>

### Fixed <a name="v2.0.2/fixed"></a>

- `README.md` has had invalid code, which as been edited as in `examples`.

### Deprecated <a name="v2.0.2/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.1][github/self/v2.0.1] <a name="v2.0.1"></a>

### Changed <a name="v2.0.1/changed"></a>

- Changelog has been edited accordingly.


### Deprecated <a name="v2.0.1/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)


## [v2.0.0][github/self/v2.0.0] <a name="v2.0.0"></a>

### Added <a name="v2.0.0/added"></a>

- Add access to a bar's `progress`.
- Extend `BernoulliProgress`
  - Implement some useful traits like `Add` and `Copy`.
  - Implement automatic `into()` for tuples (`(successes, attempts)`) or successes (`u32` or `bool`).
  - Implement constructing method `new()` and `Default`.


### Changed <a name="v2.0.0/changed"></a>

- `ID`s in `CHANGELOG.md` since underscores (`_`) are preferred over dots (`.`) in `URL`s, but `v1103` could stand for `v11.0.3` and `v1.10.3`.


### Deprecated <a name="v2.0.0/deprecated"></a>

- Detailled documentation is missing, though examples are good.
- Bar-styles to easily and safely configure bar-styles (e.g. `[====>   ]` -> `[----o   ]`)
- Changelog has been forgotten to adapt..


## [v1.0.3][github/self/v1.0.3] <a name="v1.0.3"></a>

### Added <a name="v1.0.3/added"></a>

- Implement `CHANGELOG.md`


### Deprecated <a name="v1.0.3/deprecated"></a>

- `BernoulliProgress` has no constructing method like `new()` and no default-implementation.
- Detailled documentation is missing, though examples are good.


## [v1.0.2][github/self/v1.0.2] <a name="v1.0.2"></a>

### Added <a name="v1.0.2/added"></a>

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

[github/self/unreleased]: https://github.com/dominicparga/progressing/compare/v2.2.2...HEAD
[github/self/v2.2.2]: https://github.com/dominicparga/progressing/compare/v2.2.1...v2.2.2
[github/self/v2.2.1]: https://github.com/dominicparga/progressing/compare/v2.2.0...v2.2.1
[github/self/v2.2.0]: https://github.com/dominicparga/progressing/compare/v2.1.0...v2.2.0
[github/self/v2.1.0]: https://github.com/dominicparga/progressing/compare/v2.0.2...v2.1.0
[github/self/v2.0.2]: https://github.com/dominicparga/progressing/compare/v2.0.1...v2.0.2
[github/self/v2.0.1]: https://github.com/dominicparga/progressing/compare/v2.0.0...v2.0.1
[github/self/v2.0.0]: https://github.com/dominicparga/progressing/compare/v1.0.3...v2.0.0
[github/self/v1.0.3]: https://github.com/dominicparga/progressing/compare/v1.0.2...v1.0.3
[github/self/v1.0.2]: https://github.com/dominicparga/progressing/releases/tag/v1.0.2
