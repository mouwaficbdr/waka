# Exit Codes

`waka` uses standardized exit codes so scripts can react to specific error conditions.

| Code | Name             | Meaning                                            |
| ---- | ---------------- | -------------------------------------------------- |
| `0`  | `Success`        | Command completed successfully                     |
| `1`  | `GeneralError`   | An unspecified error occurred                      |
| `2`  | `UsageError`     | Invalid arguments or flags                         |
| `3`  | `AuthError`      | Not authenticated or API key is invalid (HTTP 401) |
| `4`  | `NetworkError`   | Could not connect to the WakaTime API              |
| `5`  | `RateLimitError` | API rate limit exceeded (HTTP 429)                 |
| `6`  | `NotFoundError`  | Requested resource not found (HTTP 404)            |
| `7`  | `ServerError`    | WakaTime API returned a 5xx error                  |
| `8`  | `CacheError`     | Local cache read/write failure                     |
| `9`  | `ConfigError`    | Configuration file is invalid or unreadable        |
| `10` | `IoError`        | File system I/O error (e.g. report output path)    |

## Usage in shell scripts

```sh
waka today
case $? in
    0) echo "OK" ;;
    3) echo "Not authenticated — run: waka auth login" ;;
    4) echo "Network error — check your connection" ;;
    5) echo "Rate limited — try again later" ;;
    *) echo "Unexpected error ($?)" ;;
esac
```
