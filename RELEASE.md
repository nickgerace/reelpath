# Release

This document contains all information related to release.

## Preparation

- [ ] Change the `version` field in `Cargo.toml` to `<new-tag>`
- [ ] Run the commands and verify that everything looks/works as expected:

```sh
cargo update
cargo +nightly fmt --all -- --check
cargo clippy -- -D warnings
cargo test -- --nocapture
cargo build --release
```

- [ ] Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
- [ ] Test the publishing workflow:

```sh
cargo publish --dry-run
```

- [ ] Finally, push (or merge) the preparation commit.

## Tagging and Publishing

- [ ] Once the prepation commit has been pushed (or merged) into `main`, execute the following commands:

```sh
git tag <new-tag>
git push --tags origin main
```

- [ ] Now, publish the crate.

```sh
cargo publish
```

- [ ] Check the [crate](https://crates.io/crates/reelpath) on `crates.io`.
- [ ] Check the [docs](https://docs.rs/reelpath) on `docs.rs`.

## Updating the Homebrew Tap

- [ ] Update the formula for the [tap](https://github.com/nickgerace/homebrew-reelpath).
