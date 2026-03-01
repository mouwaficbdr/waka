# waka languages

Browse which programming languages you use most.

## Subcommands

| Subcommand            | Description                             |
| --------------------- | --------------------------------------- |
| `waka languages list` | List all languages sorted by time coded |
| `waka languages top`  | Show only the top N languages           |

## Usage

```sh
waka languages <SUBCOMMAND> [OPTIONS]
```

## Options

| Flag                    | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `-f, --format <FORMAT>` | Output format: `table` (default), `json`, `csv`, `plain` |
| `--no-cache`            | Bypass the local cache                                   |

## Examples

```sh
waka languages list
waka languages top
waka languages list --format json
```
