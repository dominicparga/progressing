# progressing

[![Tag][github/tags/badge]][github/tags]
[![Crates.io][crates.io/progressing/badge]][crates.io/progressing]
[![Docs][docs.rs/progressing/badge]][docs.rs/progressing]

[![Changelog][github/blob/changelog/badge]][github/blob/changelog]
[![Last commit][github/last-commit/badge]][github/last-commit]

[![License][github/license/badge]][github/license]


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


[crates.io/progressing]: https://crates.io/crates/progressing
[crates.io/progressing/badge]: https://img.shields.io/crates/v/progressing?style=for-the-badge
[docs.rs/progressing]: https://docs.rs/progressing/
[docs.rs/progressing/badge]: https://img.shields.io/crates/v/progressing?color=informational&label=docs&style=for-the-badge
[github/blob/changelog]: https://github.com/dominicparga/progressing/blob/master/CHANGELOG.md
[github/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-master-blueviolet?style=for-the-badge
[github/last-commit]: https://github.com/dominicparga/progressing/commits
[github/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/progressing?style=for-the-badge
[github/license]: https://github.com/dominicparga/progressing/blob/master/LICENSE
[github/license/badge]: https://img.shields.io/github/license/dominicparga/progressing?style=for-the-badge
[github/tags]: https://github.com/dominicparga/progressing/tags
[github/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/progressing?sort=semver&style=for-the-badge
[github/tree/examples]: https://github.com/dominicparga/progressing/tree/master/examples
