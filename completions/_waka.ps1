
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'waka' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'waka'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'waka' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('auth', 'auth', [CompletionResultType]::ParameterValue, 'Manage API key and authentication')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Show coding statistics')
            [CompletionResult]::new('projects', 'projects', [CompletionResultType]::ParameterValue, 'Browse and filter projects')
            [CompletionResult]::new('languages', 'languages', [CompletionResultType]::ParameterValue, 'Browse coding languages')
            [CompletionResult]::new('editors', 'editors', [CompletionResultType]::ParameterValue, 'Browse editors and IDEs')
            [CompletionResult]::new('goals', 'goals', [CompletionResultType]::ParameterValue, 'View and watch coding goals')
            [CompletionResult]::new('leaderboard', 'leaderboard', [CompletionResultType]::ParameterValue, 'View the `WakaTime` leaderboard')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Generate productivity reports')
            [CompletionResult]::new('dashboard', 'dashboard', [CompletionResultType]::ParameterValue, 'Launch the interactive TUI dashboard')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'Shell prompt integration (reads from cache only, no network)')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Manage waka configuration')
            [CompletionResult]::new('cache', 'cache', [CompletionResultType]::ParameterValue, 'Manage the local response cache')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;auth' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Log in with your `WakaTime` API key')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove the stored API key')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show whether you are currently logged in')
            [CompletionResult]::new('show-key', 'show-key', [CompletionResultType]::ParameterValue, 'Display the stored API key (masked by default)')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to a different profile')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;auth;login' {
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'Provide the API key directly (non-interactive)')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Profile to store credentials under')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;auth;logout' {
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Log out a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;auth;status' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;auth;show-key' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;auth;switch' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;auth;help' {
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Log in with your `WakaTime` API key')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove the stored API key')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show whether you are currently logged in')
            [CompletionResult]::new('show-key', 'show-key', [CompletionResultType]::ParameterValue, 'Display the stored API key (masked by default)')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to a different profile')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;auth;help;login' {
            break
        }
        'waka;auth;help;logout' {
            break
        }
        'waka;auth;help;status' {
            break
        }
        'waka;auth;help;show-key' {
            break
        }
        'waka;auth;help;switch' {
            break
        }
        'waka;auth;help;help' {
            break
        }
        'waka;stats' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('today', 'today', [CompletionResultType]::ParameterValue, 'Show today''s coding activity')
            [CompletionResult]::new('yesterday', 'yesterday', [CompletionResultType]::ParameterValue, 'Show yesterday''s coding activity')
            [CompletionResult]::new('week', 'week', [CompletionResultType]::ParameterValue, 'Show the last 7 days of activity')
            [CompletionResult]::new('month', 'month', [CompletionResultType]::ParameterValue, 'Show the last 30 days of activity')
            [CompletionResult]::new('year', 'year', [CompletionResultType]::ParameterValue, 'Show the last 365 days of activity')
            [CompletionResult]::new('range', 'range', [CompletionResultType]::ParameterValue, 'Show activity for a custom date range')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;stats;today' {
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;yesterday' {
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;week' {
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;month' {
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;year' {
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;range' {
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'Start date (YYYY-MM-DD)')
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'End date (YYYY-MM-DD)')
            [CompletionResult]::new('--project', '--project', [CompletionResultType]::ParameterName, 'Filter by project name')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'Filter by language')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;stats;help' {
            [CompletionResult]::new('today', 'today', [CompletionResultType]::ParameterValue, 'Show today''s coding activity')
            [CompletionResult]::new('yesterday', 'yesterday', [CompletionResultType]::ParameterValue, 'Show yesterday''s coding activity')
            [CompletionResult]::new('week', 'week', [CompletionResultType]::ParameterValue, 'Show the last 7 days of activity')
            [CompletionResult]::new('month', 'month', [CompletionResultType]::ParameterValue, 'Show the last 30 days of activity')
            [CompletionResult]::new('year', 'year', [CompletionResultType]::ParameterValue, 'Show the last 365 days of activity')
            [CompletionResult]::new('range', 'range', [CompletionResultType]::ParameterValue, 'Show activity for a custom date range')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;stats;help;today' {
            break
        }
        'waka;stats;help;yesterday' {
            break
        }
        'waka;stats;help;week' {
            break
        }
        'waka;stats;help;month' {
            break
        }
        'waka;stats;help;year' {
            break
        }
        'waka;stats;help;range' {
            break
        }
        'waka;stats;help;help' {
            break
        }
        'waka;projects' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all projects with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the most active projects')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show detailed stats for a project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;projects;list' {
            [CompletionResult]::new('--sort-by', '--sort-by', [CompletionResultType]::ParameterName, 'Sort field')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Maximum number of results')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;projects;top' {
            [CompletionResult]::new('--period', '--period', [CompletionResultType]::ParameterName, 'Time period to aggregate over')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;projects;show' {
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'Start date (YYYY-MM-DD)')
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'End date (YYYY-MM-DD)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;projects;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all projects with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the most active projects')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show detailed stats for a project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;projects;help;list' {
            break
        }
        'waka;projects;help;top' {
            break
        }
        'waka;projects;help;show' {
            break
        }
        'waka;projects;help;help' {
            break
        }
        'waka;languages' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all languages with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top languages')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;languages;list' {
            [CompletionResult]::new('--period', '--period', [CompletionResultType]::ParameterName, 'Time period to aggregate over')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;languages;top' {
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Maximum number of results')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;languages;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all languages with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top languages')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;languages;help;list' {
            break
        }
        'waka;languages;help;top' {
            break
        }
        'waka;languages;help;help' {
            break
        }
        'waka;editors' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all editors with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top editors')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;editors;list' {
            [CompletionResult]::new('--period', '--period', [CompletionResultType]::ParameterName, 'Time period to aggregate over')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;editors;top' {
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Maximum number of results')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;editors;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all editors with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top editors')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;editors;help;list' {
            break
        }
        'waka;editors;help;top' {
            break
        }
        'waka;editors;help;help' {
            break
        }
        'waka;goals' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all active goals')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show details for a specific goal')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch goals and refresh periodically')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;goals;list' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;goals;show' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;goals;watch' {
            [CompletionResult]::new('--interval', '--interval', [CompletionResultType]::ParameterName, 'Refresh interval in seconds')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--notify', '--notify', [CompletionResultType]::ParameterName, 'Send a desktop notification when a goal is reached')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;goals;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all active goals')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show details for a specific goal')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch goals and refresh periodically')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;goals;help;list' {
            break
        }
        'waka;goals;help;show' {
            break
        }
        'waka;goals;help;watch' {
            break
        }
        'waka;goals;help;help' {
            break
        }
        'waka;leaderboard' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show the public leaderboard')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;leaderboard;show' {
            [CompletionResult]::new('--page', '--page', [CompletionResultType]::ParameterName, 'Page number')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;leaderboard;help' {
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show the public leaderboard')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;leaderboard;help;show' {
            break
        }
        'waka;leaderboard;help;help' {
            break
        }
        'waka;report' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate a productivity report for a date range')
            [CompletionResult]::new('summary', 'summary', [CompletionResultType]::ParameterValue, 'Show a brief productivity summary')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;report;generate' {
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'Start date (YYYY-MM-DD)')
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'End date (YYYY-MM-DD)')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file path')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Report format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Report format')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;report;summary' {
            [CompletionResult]::new('--period', '--period', [CompletionResultType]::ParameterName, 'Period to summarise')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;report;help' {
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate a productivity report for a date range')
            [CompletionResult]::new('summary', 'summary', [CompletionResultType]::ParameterValue, 'Show a brief productivity summary')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;report;help;generate' {
            break
        }
        'waka;report;help;summary' {
            break
        }
        'waka;report;help;help' {
            break
        }
        'waka;dashboard' {
            [CompletionResult]::new('--refresh', '--refresh', [CompletionResultType]::ParameterName, 'Auto-refresh interval in seconds')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;prompt' {
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output style')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;completions' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get the value of a config key')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the value of a config key')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Open the config file in $EDITOR')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the config file')
            [CompletionResult]::new('reset', 'reset', [CompletionResultType]::ParameterValue, 'Reset config to defaults')
            [CompletionResult]::new('doctor', 'doctor', [CompletionResultType]::ParameterValue, 'Run a full diagnostic check')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;config;get' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;set' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;edit' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;path' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;reset' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--confirm', '--confirm', [CompletionResultType]::ParameterName, 'Skip the confirmation prompt')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;doctor' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;config;help' {
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get the value of a config key')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the value of a config key')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Open the config file in $EDITOR')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the config file')
            [CompletionResult]::new('reset', 'reset', [CompletionResultType]::ParameterValue, 'Reset config to defaults')
            [CompletionResult]::new('doctor', 'doctor', [CompletionResultType]::ParameterValue, 'Run a full diagnostic check')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;config;help;get' {
            break
        }
        'waka;config;help;set' {
            break
        }
        'waka;config;help;edit' {
            break
        }
        'waka;config;help;path' {
            break
        }
        'waka;config;help;reset' {
            break
        }
        'waka;config;help;doctor' {
            break
        }
        'waka;config;help;help' {
            break
        }
        'waka;cache' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clear all cached entries (or only those older than a duration)')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Show cache statistics (entry count, disk size, last write)')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the cache directory')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;cache;clear' {
            [CompletionResult]::new('--older', '--older', [CompletionResultType]::ParameterName, 'Remove only entries older than this duration (e.g. `1h`, `24h`, `7d`)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;cache;info' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;cache;path' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('--profile', '--profile', [CompletionResultType]::ParameterName, 'Use a specific profile')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format: table, json, csv, plain')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Skip the cache and force a fresh API request')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colors (equivalent to `NO_COLOR=1`)')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-essential output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose mode (shows HTTP requests)')
            [CompletionResult]::new('--csv-bom', '--csv-bom', [CompletionResultType]::ParameterName, 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'waka;cache;help' {
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clear all cached entries (or only those older than a duration)')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Show cache statistics (entry count, disk size, last write)')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the cache directory')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;cache;help;clear' {
            break
        }
        'waka;cache;help;info' {
            break
        }
        'waka;cache;help;path' {
            break
        }
        'waka;cache;help;help' {
            break
        }
        'waka;help' {
            [CompletionResult]::new('auth', 'auth', [CompletionResultType]::ParameterValue, 'Manage API key and authentication')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Show coding statistics')
            [CompletionResult]::new('projects', 'projects', [CompletionResultType]::ParameterValue, 'Browse and filter projects')
            [CompletionResult]::new('languages', 'languages', [CompletionResultType]::ParameterValue, 'Browse coding languages')
            [CompletionResult]::new('editors', 'editors', [CompletionResultType]::ParameterValue, 'Browse editors and IDEs')
            [CompletionResult]::new('goals', 'goals', [CompletionResultType]::ParameterValue, 'View and watch coding goals')
            [CompletionResult]::new('leaderboard', 'leaderboard', [CompletionResultType]::ParameterValue, 'View the `WakaTime` leaderboard')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Generate productivity reports')
            [CompletionResult]::new('dashboard', 'dashboard', [CompletionResultType]::ParameterValue, 'Launch the interactive TUI dashboard')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'Shell prompt integration (reads from cache only, no network)')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Manage waka configuration')
            [CompletionResult]::new('cache', 'cache', [CompletionResultType]::ParameterValue, 'Manage the local response cache')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'waka;help;auth' {
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Log in with your `WakaTime` API key')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove the stored API key')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show whether you are currently logged in')
            [CompletionResult]::new('show-key', 'show-key', [CompletionResultType]::ParameterValue, 'Display the stored API key (masked by default)')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to a different profile')
            break
        }
        'waka;help;auth;login' {
            break
        }
        'waka;help;auth;logout' {
            break
        }
        'waka;help;auth;status' {
            break
        }
        'waka;help;auth;show-key' {
            break
        }
        'waka;help;auth;switch' {
            break
        }
        'waka;help;stats' {
            [CompletionResult]::new('today', 'today', [CompletionResultType]::ParameterValue, 'Show today''s coding activity')
            [CompletionResult]::new('yesterday', 'yesterday', [CompletionResultType]::ParameterValue, 'Show yesterday''s coding activity')
            [CompletionResult]::new('week', 'week', [CompletionResultType]::ParameterValue, 'Show the last 7 days of activity')
            [CompletionResult]::new('month', 'month', [CompletionResultType]::ParameterValue, 'Show the last 30 days of activity')
            [CompletionResult]::new('year', 'year', [CompletionResultType]::ParameterValue, 'Show the last 365 days of activity')
            [CompletionResult]::new('range', 'range', [CompletionResultType]::ParameterValue, 'Show activity for a custom date range')
            break
        }
        'waka;help;stats;today' {
            break
        }
        'waka;help;stats;yesterday' {
            break
        }
        'waka;help;stats;week' {
            break
        }
        'waka;help;stats;month' {
            break
        }
        'waka;help;stats;year' {
            break
        }
        'waka;help;stats;range' {
            break
        }
        'waka;help;projects' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all projects with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the most active projects')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show detailed stats for a project')
            break
        }
        'waka;help;projects;list' {
            break
        }
        'waka;help;projects;top' {
            break
        }
        'waka;help;projects;show' {
            break
        }
        'waka;help;languages' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all languages with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top languages')
            break
        }
        'waka;help;languages;list' {
            break
        }
        'waka;help;languages;top' {
            break
        }
        'waka;help;editors' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all editors with coding time')
            [CompletionResult]::new('top', 'top', [CompletionResultType]::ParameterValue, 'Show the top editors')
            break
        }
        'waka;help;editors;list' {
            break
        }
        'waka;help;editors;top' {
            break
        }
        'waka;help;goals' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all active goals')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show details for a specific goal')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch goals and refresh periodically')
            break
        }
        'waka;help;goals;list' {
            break
        }
        'waka;help;goals;show' {
            break
        }
        'waka;help;goals;watch' {
            break
        }
        'waka;help;leaderboard' {
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show the public leaderboard')
            break
        }
        'waka;help;leaderboard;show' {
            break
        }
        'waka;help;report' {
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate a productivity report for a date range')
            [CompletionResult]::new('summary', 'summary', [CompletionResultType]::ParameterValue, 'Show a brief productivity summary')
            break
        }
        'waka;help;report;generate' {
            break
        }
        'waka;help;report;summary' {
            break
        }
        'waka;help;dashboard' {
            break
        }
        'waka;help;prompt' {
            break
        }
        'waka;help;completions' {
            break
        }
        'waka;help;config' {
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get the value of a config key')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the value of a config key')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Open the config file in $EDITOR')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the config file')
            [CompletionResult]::new('reset', 'reset', [CompletionResultType]::ParameterValue, 'Reset config to defaults')
            [CompletionResult]::new('doctor', 'doctor', [CompletionResultType]::ParameterValue, 'Run a full diagnostic check')
            break
        }
        'waka;help;config;get' {
            break
        }
        'waka;help;config;set' {
            break
        }
        'waka;help;config;edit' {
            break
        }
        'waka;help;config;path' {
            break
        }
        'waka;help;config;reset' {
            break
        }
        'waka;help;config;doctor' {
            break
        }
        'waka;help;cache' {
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clear all cached entries (or only those older than a duration)')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Show cache statistics (entry count, disk size, last write)')
            [CompletionResult]::new('path', 'path', [CompletionResultType]::ParameterValue, 'Print the path to the cache directory')
            break
        }
        'waka;help;cache;clear' {
            break
        }
        'waka;help;cache;info' {
            break
        }
        'waka;help;cache;path' {
            break
        }
        'waka;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
