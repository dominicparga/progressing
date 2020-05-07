# progressing

[![Build Status nightly][github/self/actions/badge]][github/self/actions]

[![Tag][github/self/tags/badge]][github/self/tags]
[![Crates.io][crates.io/self/badge]][crates.io/self]
[![Docs][docs.rs/self/badge]][docs.rs/self]

[![Changelog][github/self/blob/changelog/badge]][github/self/blob/changelog]
[![Last commit][github/self/last-commit/badge]][github/self/last-commit]

[![License][github/self/license/badge]][github/self/license]


## Look and feel

```rust
// Printing value 0.3 clamped to [0, 1]
// [=====>------------]
let mut progress_bar = progressing::ClampingBar::new();
progress_bar.set_bar_len(20);
println!("{}", progress_bar.set(0.3));

// Mapping from [-9, 5] to [0, 1]
// [================>-] (4 / 5)
let mut progress_bar = progressing::MappingBar::new(-9..=5);
progress_bar.set_bar_len(20);
println!("{}", progress_bar.set(4));

// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
// [============>-----] (42 / 60 # 130)
let mut progress_bar = progressing::BernoulliBar::from_goal(60);
progress_bar.set_bar_len(20);
println!("{}", progress_bar.set((42, 130)));
```


## Setup and usage

Please refer to the [examples][github/self/tree/examples].


[crates.io/self]: https://crates.io/crates/progressing
[crates.io/self/badge]: https://img.shields.io/crates/v/progressing?style=for-the-badge
[docs.rs/self]: https://docs.rs/progressing/
[docs.rs/self/badge]: https://img.shields.io/crates/v/progressing?color=informational&label=docs&style=for-the-badge
[github/self/actions]: https://github.com/dominicparga/progressing/actions
[github/self/actions/badge]: https://img.shields.io/github/workflow/status/dominicparga/progressing/Rust?label=nightly-build&style=for-the-badge
[github/self/blob/changelog]: https://github.com/dominicparga/progressing/blob/nightly/CHANGELOG.md
[github/self/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-nightly-blueviolet?style=for-the-badge
[github/self/last-commit]: https://github.com/dominicparga/progressing/commits
[github/self/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/progressing?style=for-the-badge
[github/self/license]: https://github.com/dominicparga/progressing/blob/nightly/LICENSE.md
[github/self/license/badge]: https://img.shields.io/badge/LICENSE-Apache--2.0-green?style=for-the-badge
[github/self/tags]: https://github.com/dominicparga/progressing/tags
[github/self/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/progressing?sort=semver&style=for-the-badge
[github/self/tree/examples]: https://github.com/dominicparga/progressing/tree/nightly/examples
