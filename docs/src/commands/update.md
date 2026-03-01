# waka update

Update `waka` to the latest released version.

## Usage

```sh
waka update
```

`waka` automatically detects how it was installed and uses the appropriate update mechanism:

| Install method         | Update command used                         |
| ---------------------- | ------------------------------------------- |
| Homebrew               | `brew upgrade waka`                         |
| Snap                   | `sudo snap refresh waka`                    |
| Flatpak                | `flatpak update io.github.mouwaficbdr.waka` |
| `cargo install`        | `cargo install waka --force`                |
| Manual binary download | Prints the releases URL for manual update   |

## Example

```sh
waka update
```

Output:

```
Current version: 1.0.0
Latest version:  1.1.0
Updating via Homebrew...
✓ waka updated to 1.1.0
```

If already up to date:

```
waka 1.0.0 is already the latest version.
```
