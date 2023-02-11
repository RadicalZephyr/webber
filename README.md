# About

A spider-themed web building game.

## Instructions

### Running the game

Use `cargo run`.

This repo is set up to always build with full optimizations, so
there's no need for a `--release` flag in most cases.  Dynamic linking
is enabled to ensure build times stay snappy.

To run an example, use `cargo run --example_name`, where
`example_name` is the file name of the example without the `.rs`
extension.


### Publishing your game

A build will be produced for Windows, MacOS and Linux each time a
[tag] is pushed to GitHub.

These can be found under the
[Releases] tab of
your project.

[tag]: https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/managing-commits/managing-tags
[Releases]: https://docs.github.com/en/rest/reference/releases


## Contributing

See [CONTRIBUTING.md]!

[CONTRIBUTING.md]: https://github.com/Leafwing-Studios/template-repo/blob/main/CONTRIBUTING.md


### Testing

1. Use doc tests aggressively to show how APIs should be used.  You
   can use `#` to hide a setup line from the doc tests.
2. Unit test belong near the code they are testing. Use `#[cfg(test)]`
   on the test module to ignore it during builds, and `#[test]` on the
   test functions to ensure they are run.
3. Integration tests should be stored in the top level `tests` folder,
   importing functions from `lib.rs`.

Use `cargo test` to run all tests.


### CI

The CI will:

1. Ensure the code is formatted with `cargo fmt`.
2. Ensure that the code compiles.
3. Ensure that (almost) all `clippy` lints pass.
4. Ensure all tests pass on Windows, MacOS and Ubuntu.

Check this locally with:

1. `cargo run -p ci`
2. `cargo test --workspace`

To manually rerun CI:

1. Navigate to the `Actions` tab.
2. Use the dropdown menu in the CI run of interest and select "View
   workflow file".
3. In the top-right corner, select "Rerun workflow".


### Documentation

Reference documentation is handled with standard Rust doc strings.
Use `cargo doc --open` to build and then open the docs.

Design docs (or other book-format documentation) is handled with
[mdBook].  Install it with `cargo install mdbook`, then use `mdbook
serve --open` to launch the docs.

[mdBook]: https://rust-lang.github.io/mdBook/index.html

### Benchmarking

To run the benchmarks, use `cargo bench`.

For more documentation on making your own benchmarks, check out
[criterion's docs][criterion-docs].

[criterion-docs]: https://bheisler.github.io/criterion.rs/book/index.html
