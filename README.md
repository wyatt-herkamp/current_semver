# current_semver!

This will parse the current version of your crate and create the semver::Version struct. 

## Usage

```rust
pub fn main() {
    let version: semver::Version = current_semver!();
}
```

Example output
```rust
pub fn main() {
    let version = semver::Version { major: 1, minor: 1, patch: 0, pre: semver::Prerelease::new("BETA").unwrap_or_default(), build: semver::BuildMetadata::default() };
}
```
