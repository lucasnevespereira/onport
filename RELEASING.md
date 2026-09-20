# Releasing onport

Releases have two parts: `cargo publish` uploads the CLI to crates.io, and pushing a `v*` tag runs [the GitHub release workflow](.github/workflows/release.yml) to build and attach macOS and Linux binaries. The workflow does not publish to crates.io.

After the release PR has merged, run these commands from a clean checkout:

```bash
git switch main
git pull --ff-only
```

Update the `version` in `Cargo.toml` (for example, from `0.1.3` to `0.2.0`). Then update the lockfile, test, and commit the version bump:

```bash
cargo check
cargo test
git add Cargo.toml Cargo.lock
git commit -m "chore(release): 0.2.0"
git push origin main
```

Wait for CI on `main` to pass. Check the package and publish it to crates.io (run `cargo login` first if needed):

```bash
cargo publish --dry-run
cargo publish
```

Finally, tag the same commit. Pushing the tag creates the GitHub release and uploads the binaries:

```bash
git tag v0.2.0
git push origin v0.2.0
```

Use the same version in `Cargo.toml`, the release commit, and the tag. For the change that removes confirmation from `onport kill`, `0.2.0` is the recommended next version.
