# progressing

[![Build Status master][github/self/actions/badge]][github/self/actions]

[![Tag][github/self/tags/badge]][github/self/tags]
[![Crates.io][crates.io/self/badge]][crates.io/self]
[![Docs][docs.rs/self/badge]][docs.rs/self]

[![Changelog][github/self/blob/changelog/badge]][github/self/blob/changelog]
[![Last commit][github/self/last-commit/badge]][github/self/last-commit]

[![License][github/self/license/badge]][github/self/license]


## Look and feel

```rust
// Printing value 0.3 clamped to [0, 1]
// [=====>            ]
let mut progressbar = progressing::ClampingBar::new();
progressbar.set_bar_len(20);
progressbar.set(0.3).reprintln()

// Mapping from [-9, 5] to [0, 1]
// [================> ] (4 / 5)
let mut progressbar = progressing::MappingBar::new(-9..=5);
progressbar.set_bar_len(20);
progressbar.set(4).reprintln()

// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
// [============>     ] (42 / 60 # 130)
let mut progressbar = progressing::BernoulliBar::from_goal(60);
progressbar.set_bar_len(20);
progressbar.set((42, 130)).reprintln()
```


## Setup and usage

Please refer to the [examples][github/tree/examples].


[crates.io/self]: https://crates.io/crates/progressing
[crates.io/self/badge]: https://img.shields.io/crates/v/progressing?style=for-the-badge
[docs.rs/self]: https://docs.rs/progressing/
[docs.rs/self/badge]: https://img.shields.io/crates/v/progressing?color=informational&label=docs&style=for-the-badge
[github/self/actions]: https://github.com/dominicparga/progressing/actions
[github/self/actions/badge]: https://img.shields.io/github/workflow/status/dominicparga/progressing/Rust?label=master-build&style=for-the-badge
[github/self/blob/changelog]: https://github.com/dominicparga/progressing/blob/master/CHANGELOG.md
[github/self/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-master-blueviolet?style=for-the-badge
[github/self/last-commit]: https://github.com/dominicparga/progressing/commits
[github/self/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/progressing?style=for-the-badge
[github/self/license]: https://github.com/dominicparga/progressing/blob/master/LICENSE
[github/self/license/badge]: https://img.shields.io/github/license/dominicparga/progressing?style=for-the-badge
[github/self/tags]: https://github.com/dominicparga/progressing/tags
[github/self/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/progressing?sort=semver&style=for-the-badge
[github/self/tree/examples]: https://github.com/dominicparga/progressing/tree/master/examples
