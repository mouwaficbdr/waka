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
Current version: 0.4.0
Latest version:  0.5.0
Updating via Homebrew...
✓ waka updated to 0.5.0
```

If already up to date:

```
waka 0.4.0 is already the latest version.
```
