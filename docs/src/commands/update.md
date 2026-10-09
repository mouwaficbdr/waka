# waka update

Update `waka` to the latest released version.

## Usage

```sh
waka update
```

`waka` checks the latest GitHub release and:

| Install method         | Behaviour                                                        |
| ---------------------- | ---------------------------------------------------------------- |
| Homebrew               | Prints `brew upgrade waka` for you to run                        |
| Pre-built binary       | Downloads the release archive, verifies it, and replaces the current binary |

## Integrity

Every release publishes a `SHA256SUMS` file. `waka update` downloads it first and only installs
the archive if its SHA-256 digest matches; otherwise nothing is changed. Releases also carry signed
build provenance, which you can check for any downloaded archive with the GitHub CLI:

```sh
gh attestation verify waka-v2.1.0-x86_64-unknown-linux-gnu.tar.gz --repo mouwaficbdr/waka
```

If the binary lives in a directory you cannot write to (e.g. `/usr/local/bin`),
run the command with the required privileges or reinstall manually.

## Update notifications

Once a day, other commands check for a newer release in the background and
print a one-line notice on stderr. Disable it with `waka config set
core.update_check false` or the `WAKA_NO_UPDATE_CHECK` environment variable.
