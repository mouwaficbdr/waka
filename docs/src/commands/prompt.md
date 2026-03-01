# waka prompt

Emit a compact coding summary for use in your shell prompt.

This command reads **only from the local cache** — it never makes a network request.
This makes it safe to call from `$PROMPT_COMMAND` or equivalent without slowing down your terminal.

## Usage

```sh
waka prompt [OPTIONS]
```

## Options

| Flag                    | Description                                       |
| ----------------------- | ------------------------------------------------- |
| `-f, --format <FORMAT>` | Output format: `table` (default), `json`, `plain` |
| `--quiet`               | Output nothing if no cached data is available     |

## Shell integration examples

```sh
# Bash — add to ~/.bashrc
PS1='[\u@\h \W $(waka prompt --quiet)] \$ '

# Zsh — add to ~/.zshrc
RPROMPT='$(waka prompt --quiet)'

# Fish — add to ~/.config/fish/config.fish
function fish_right_prompt
    waka prompt --quiet
end
```

## Notes

- If the cache is empty or stale, `waka prompt` outputs nothing (with `--quiet`) or a placeholder.
- Run `waka stats today` first to populate the cache.
