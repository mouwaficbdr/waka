# waka completions

Generate shell completion scripts for `waka`.

## Usage

```sh
waka completions <SHELL>
```

## Supported shells

| Shell        |
| ------------ |
| `bash`       |
| `zsh`        |
| `fish`       |
| `powershell` |
| `elvish`     |

## Installation

```sh
# Bash
waka completions bash > ~/.local/share/bash-completion/completions/waka

# Zsh
waka completions zsh > "${fpath[1]}/_waka"

# Fish
waka completions fish > ~/.config/fish/completions/waka.fish

# PowerShell
waka completions powershell >> $PROFILE
```

After installing, restart your shell or source the relevant file to enable completions.
