# Shell Completions

`waka` can generate tab-completion scripts for Bash, Zsh, Fish, PowerShell, and Elvish.

## Generate a completion script

```sh
waka completions <SHELL> [--output <FILE>]
```

### Bash

```sh
waka completions bash > ~/.local/share/bash-completion/completions/waka
```

Or, for system-wide installation:

```sh
waka completions bash | sudo tee /etc/bash_completion.d/waka
```

Then reload your shell:

```sh
source ~/.bashrc
```

### Zsh

```sh
waka completions zsh > "${fpath[1]}/_waka"
```

Add this to your `~/.zshrc` if not already present:

```sh
autoload -U compinit && compinit
```

### Fish

```sh
waka completions fish > ~/.config/fish/completions/waka.fish
```

### PowerShell

```powershell
waka completions powershell | Out-File -Encoding utf8 ~\Documents\PowerShell\completions\waka.ps1
```

Add to your `$PROFILE`:

```powershell
. ~\Documents\PowerShell\completions\waka.ps1
```

### Elvish

```sh
waka completions elvish > ~/.config/elvish/lib/waka.elv
```

## Homebrew

If you installed via Homebrew, completions are set up automatically.
Run `brew info waka` to see the installed completion paths.
