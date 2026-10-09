# waka prompt

Emit a compact coding summary for use in your shell prompt.

This command reads **only from the local cache** — it never makes a network request.
This makes it safe to call from `$PROMPT_COMMAND` or equivalent without slowing down your terminal.

## Usage

```sh
waka prompt [--style simple|detailed]
```

## Options

| Flag              | Description                                          |
| ----------------- | ---------------------------------------------------- |
| `--style simple`  | Today's total, e.g. `⏱ 6h 42m` (default)             |
| `--style detailed`| Total and top project, e.g. `⏱ 6h 42m \| my-saas`   |

`waka prompt` prints nothing when there is no data for today, so it never clutters your prompt.

## Shell integration examples

```sh
# Bash — add to ~/.bashrc
PS1='[\u@\h \W $(waka prompt)] \$ '

# Zsh — add to ~/.zshrc
RPROMPT='$(waka prompt)'

# Fish — add to ~/.config/fish/config.fish
function fish_right_prompt
    waka prompt
end
```

## Notes

- The value comes from the last `waka stats today` of the day, whatever its age: run
  `waka stats today` (or schedule it, e.g. every 15 minutes) to refresh it.
- Data from a previous day is never shown.
