# waka editors

Browse which editors and IDEs you use most.

## Subcommands

| Subcommand          | Description                           |
| ------------------- | ------------------------------------- |
| `waka editors list` | List all editors sorted by time coded |
| `waka editors top`  | Show only the top N editors           |

## Usage

```sh
waka editors <SUBCOMMAND> [OPTIONS]
```

## Options

| Flag                    | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `-f, --format <FORMAT>` | Output format: `table` (default), `json`, `csv`, `plain` |
| `--no-cache`            | Bypass the local cache                                   |

## Examples

```sh
waka editors list
waka editors top
waka editors list --format json
```
