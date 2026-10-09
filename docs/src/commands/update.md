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
| Pre-built binary       | Downloads the release archive and replaces the current binary   |

If the binary lives in a directory you cannot write to (e.g. `/usr/local/bin`),
run the command with the required privileges or reinstall manually.

## Update notifications

Once a day, other commands check for a newer release in the background and
print a one-line notice on stderr. Disable it with `waka config set
core.update_check false` or the `WAKA_NO_UPDATE_CHECK` environment variable.
