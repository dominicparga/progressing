# progressing

[![Build Status nightly][github/self/actions/badge]][github/self/actions]

[![Tag][github/self/tags/badge]][github/self/tags]
[![Crates.io][crates.io/self/badge]][crates.io/self]
[![Docs][docs.rs/self/badge]][docs.rs/self]

[![Changelog][github/self/blob/changelog/badge]][github/self/blob/changelog]
[![Last commit][github/self/last-commit/badge]][github/self/last-commit]

[![License][github/self/license/badge]][github/self/license]


## Look and feel

At first, the trait `Baring` is needed.

```rust
use progressing::{
    // The underlying Trait
    Baring,
    // Just handy names for the examples below
    bernoulli::Bar as BernoulliBar,
    clamping::Bar as ClampingBar,
    mapping::Bar as MappingBar,
};
```

In the following, different use-cases of the provided progress-bars are presented.
Note, that the examples below use `set(...)`, but `add(...)` is supported as well.

- Printing value `0.3` clamped to `[0, 1]` prints `[=====>------------]`.

  ```rust
  let mut progress_bar = ClampingBar::new();
  progress_bar.set_len(20);
  progress_bar.set(0.3);
  println!("{}", progress_bar);
  ```

- Printing value `4` mapped from `[-9, 5]` to `[0, 1]` prints `[================>-] (4 / 5)`.

  ```rust
  let mut progress_bar = MappingBar::with_range(-9, 5);
  progress_bar.set_len(20);
  progress_bar.set(4);
  println!("{}", progress_bar);
  ```

- Every bar can be used with a simple time-approximation based on the past process.
  For a process of this duration, this example would print `[================>-] (4 / 5) ~ 2 min`.
  The only difference is the call of `timed()`.

  ```rust
  let mut progress_bar = MappingBar::with_range(-9, 5).timed();
  progress_bar.set_len(20);
  progress_bar.set(4);
  println!("{}", progress_bar);
  ```

- In case something should be counted and failures may occur, try this example.
  When counting `42` successes, where `60` is the goal and `130` attempts have been made, `[============>-----] (42 / 60 # 130)` is printed.
  Adding trials may be handier using `bool`s.

  ```rust
  let mut progress_bar = BernoulliBar::from_goal(60);
  progress_bar.set_len(20);
  progress_bar.set((42, 130));
  println!("{}", progress_bar);

  let is_successful = true;
  if is_successful {
      // Does increase both 42 and 130
      progress_bar.add(true);
  } else {
      // Does increase 130 only
      progress_bar.add(false);
  }
  ```

- You may change a bar's style by setting it to a string of 5 characters.

  ```rust
  let mut progress_bar = ClampingBar::new();
  progress_bar.set_len(20);
  progress_bar.set(0.3);

  // different custom styles are possible

  // prints (----->............)
  progress_bar.set_style("(->.)");
  println!("{}", progress_bar);

  // prints [#####             ]
  progress_bar.set_style("[#  ]");
  println!("{}", progress_bar);

  // prints (#####-------------)
  progress_bar.set_style("(#--)");
  println!("{}", progress_bar);
  ```

- Another typical use-case may be printing some, not every progress in a loop.

  ```rust
  let mut progress_bar = BernoulliBar::with_goal(100).timed();
  progress_bar.set_len(20);
  progress_bar.set(13);

  // do the job and show progress
  for _ in 0..100 {
      progress_bar.add(true);
      if progress_bar.has_progressed_significantly() {
          progress_bar.remember_significant_progress();
          println!("{}", progress_bar);
      }

      std::thread::sleep(std::time::Duration::from_millis(100));
  }
  println!("{}", progress_bar);
  ```

  will print

  ```text
  [=>................] (10/100) #14 ~8s
  [===>..............] (20/100) #20 ~7s
  [=====>............] (30/100) #30 ~6s
  [=======>..........] (40/100) #40 ~5s
  [=========>........] (50/100) #50 ~4s
  [==========>.......] (60/100) #60 ~3s
  [============>.....] (70/100) #70 ~2s
  [==============>...] (80/100) #80 ~1s
  [================>.] (90/100) #90 ~0s
  [==================] (100/100) #100 ~0s
  [==================] (100/100) #113 ~0s
  ```

  A line is printed every time when another `10 %` of the goal is reached.
  Please note, that the progress-bar starts with 13 and hence needs 113 attempts in total.


## Setup and usage

Just add `progressing = '3'` to the dependencies in `Cargo.toml`.

Please refer to the [examples][github/self/tree/examples] for some more examples.


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
