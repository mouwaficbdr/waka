
use builtin;
use str;

set edit:completion:arg-completer[waka] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'waka'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'waka'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand auth 'Manage API key and authentication'
            cand stats 'Show coding statistics'
            cand projects 'Browse and filter projects'
            cand languages 'Browse coding languages'
            cand editors 'Browse editors and IDEs'
            cand goals 'View and watch coding goals'
            cand leaderboard 'View the `WakaTime` leaderboard'
            cand report 'Generate productivity reports'
            cand dashboard 'Launch the interactive TUI dashboard'
            cand prompt 'Shell prompt integration (reads from cache only, no network)'
            cand completions 'Generate shell completions'
            cand config 'Manage waka configuration'
            cand cache 'Manage the local response cache'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;auth'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand login 'Log in with your `WakaTime` API key'
            cand logout 'Remove the stored API key'
            cand status 'Show whether you are currently logged in'
            cand show-key 'Display the stored API key (masked by default)'
            cand switch 'Switch to a different profile'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;auth;login'= {
            cand --api-key 'Provide the API key directly (non-interactive)'
            cand --profile 'Profile to store credentials under'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;auth;logout'= {
            cand --profile 'Log out a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;auth;status'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;auth;show-key'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;auth;switch'= {
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;auth;help'= {
            cand login 'Log in with your `WakaTime` API key'
            cand logout 'Remove the stored API key'
            cand status 'Show whether you are currently logged in'
            cand show-key 'Display the stored API key (masked by default)'
            cand switch 'Switch to a different profile'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;auth;help;login'= {
        }
        &'waka;auth;help;logout'= {
        }
        &'waka;auth;help;status'= {
        }
        &'waka;auth;help;show-key'= {
        }
        &'waka;auth;help;switch'= {
        }
        &'waka;auth;help;help'= {
        }
        &'waka;stats'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand today 'Show today''s coding activity'
            cand yesterday 'Show yesterday''s coding activity'
            cand week 'Show the last 7 days of activity'
            cand month 'Show the last 30 days of activity'
            cand year 'Show the last 365 days of activity'
            cand range 'Show activity for a custom date range'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;stats;today'= {
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;yesterday'= {
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;week'= {
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;month'= {
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;year'= {
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;range'= {
            cand --from 'Start date (YYYY-MM-DD)'
            cand --to 'End date (YYYY-MM-DD)'
            cand --project 'Filter by project name'
            cand --language 'Filter by language'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;stats;help'= {
            cand today 'Show today''s coding activity'
            cand yesterday 'Show yesterday''s coding activity'
            cand week 'Show the last 7 days of activity'
            cand month 'Show the last 30 days of activity'
            cand year 'Show the last 365 days of activity'
            cand range 'Show activity for a custom date range'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;stats;help;today'= {
        }
        &'waka;stats;help;yesterday'= {
        }
        &'waka;stats;help;week'= {
        }
        &'waka;stats;help;month'= {
        }
        &'waka;stats;help;year'= {
        }
        &'waka;stats;help;range'= {
        }
        &'waka;stats;help;help'= {
        }
        &'waka;projects'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List all projects with coding time'
            cand top 'Show the most active projects'
            cand show 'Show detailed stats for a project'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;projects;list'= {
            cand --sort-by 'Sort field'
            cand --limit 'Maximum number of results'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;projects;top'= {
            cand --period 'Time period to aggregate over'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;projects;show'= {
            cand --from 'Start date (YYYY-MM-DD)'
            cand --to 'End date (YYYY-MM-DD)'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;projects;help'= {
            cand list 'List all projects with coding time'
            cand top 'Show the most active projects'
            cand show 'Show detailed stats for a project'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;projects;help;list'= {
        }
        &'waka;projects;help;top'= {
        }
        &'waka;projects;help;show'= {
        }
        &'waka;projects;help;help'= {
        }
        &'waka;languages'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List all languages with coding time'
            cand top 'Show the top languages'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;languages;list'= {
            cand --period 'Time period to aggregate over'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;languages;top'= {
            cand --limit 'Maximum number of results'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;languages;help'= {
            cand list 'List all languages with coding time'
            cand top 'Show the top languages'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;languages;help;list'= {
        }
        &'waka;languages;help;top'= {
        }
        &'waka;languages;help;help'= {
        }
        &'waka;editors'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List all editors with coding time'
            cand top 'Show the top editors'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;editors;list'= {
            cand --period 'Time period to aggregate over'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;editors;top'= {
            cand --limit 'Maximum number of results'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;editors;help'= {
            cand list 'List all editors with coding time'
            cand top 'Show the top editors'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;editors;help;list'= {
        }
        &'waka;editors;help;top'= {
        }
        &'waka;editors;help;help'= {
        }
        &'waka;goals'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand list 'List all active goals'
            cand show 'Show details for a specific goal'
            cand watch 'Watch goals and refresh periodically'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;goals;list'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;goals;show'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;goals;watch'= {
            cand --interval 'Refresh interval in seconds'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --notify 'Send a desktop notification when a goal is reached'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;goals;help'= {
            cand list 'List all active goals'
            cand show 'Show details for a specific goal'
            cand watch 'Watch goals and refresh periodically'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;goals;help;list'= {
        }
        &'waka;goals;help;show'= {
        }
        &'waka;goals;help;watch'= {
        }
        &'waka;goals;help;help'= {
        }
        &'waka;leaderboard'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand show 'Show the public leaderboard'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;leaderboard;show'= {
            cand --page 'Page number'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;leaderboard;help'= {
            cand show 'Show the public leaderboard'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;leaderboard;help;show'= {
        }
        &'waka;leaderboard;help;help'= {
        }
        &'waka;report'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand generate 'Generate a productivity report for a date range'
            cand summary 'Show a brief productivity summary'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;report;generate'= {
            cand --from 'Start date (YYYY-MM-DD)'
            cand --to 'End date (YYYY-MM-DD)'
            cand -o 'Output file path'
            cand --output 'Output file path'
            cand -f 'Report format'
            cand --format 'Report format'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;report;summary'= {
            cand --period 'Period to summarise'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;report;help'= {
            cand generate 'Generate a productivity report for a date range'
            cand summary 'Show a brief productivity summary'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;report;help;generate'= {
        }
        &'waka;report;help;summary'= {
        }
        &'waka;report;help;help'= {
        }
        &'waka;dashboard'= {
            cand --refresh 'Auto-refresh interval in seconds'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;prompt'= {
            cand --format 'Output style'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;completions'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand get 'Get the value of a config key'
            cand set 'Set the value of a config key'
            cand edit 'Open the config file in $EDITOR'
            cand path 'Print the path to the config file'
            cand reset 'Reset config to defaults'
            cand doctor 'Run a full diagnostic check'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;config;get'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;set'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;edit'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;path'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;reset'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --confirm 'Skip the confirmation prompt'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;doctor'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;config;help'= {
            cand get 'Get the value of a config key'
            cand set 'Set the value of a config key'
            cand edit 'Open the config file in $EDITOR'
            cand path 'Print the path to the config file'
            cand reset 'Reset config to defaults'
            cand doctor 'Run a full diagnostic check'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;config;help;get'= {
        }
        &'waka;config;help;set'= {
        }
        &'waka;config;help;edit'= {
        }
        &'waka;config;help;path'= {
        }
        &'waka;config;help;reset'= {
        }
        &'waka;config;help;doctor'= {
        }
        &'waka;config;help;help'= {
        }
        &'waka;cache'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand clear 'Clear all cached entries (or only those older than a duration)'
            cand info 'Show cache statistics (entry count, disk size, last write)'
            cand path 'Print the path to the cache directory'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;cache;clear'= {
            cand --older 'Remove only entries older than this duration (e.g. `1h`, `24h`, `7d`)'
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;cache;info'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;cache;path'= {
            cand -p 'Use a specific profile'
            cand --profile 'Use a specific profile'
            cand -f 'Output format: table, json, csv, plain'
            cand --format 'Output format: table, json, csv, plain'
            cand --no-cache 'Skip the cache and force a fresh API request'
            cand --no-color 'Disable colors (equivalent to `NO_COLOR=1`)'
            cand --quiet 'Suppress non-essential output'
            cand --verbose 'Enable verbose mode (shows HTTP requests)'
            cand --csv-bom 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'waka;cache;help'= {
            cand clear 'Clear all cached entries (or only those older than a duration)'
            cand info 'Show cache statistics (entry count, disk size, last write)'
            cand path 'Print the path to the cache directory'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;cache;help;clear'= {
        }
        &'waka;cache;help;info'= {
        }
        &'waka;cache;help;path'= {
        }
        &'waka;cache;help;help'= {
        }
        &'waka;help'= {
            cand auth 'Manage API key and authentication'
            cand stats 'Show coding statistics'
            cand projects 'Browse and filter projects'
            cand languages 'Browse coding languages'
            cand editors 'Browse editors and IDEs'
            cand goals 'View and watch coding goals'
            cand leaderboard 'View the `WakaTime` leaderboard'
            cand report 'Generate productivity reports'
            cand dashboard 'Launch the interactive TUI dashboard'
            cand prompt 'Shell prompt integration (reads from cache only, no network)'
            cand completions 'Generate shell completions'
            cand config 'Manage waka configuration'
            cand cache 'Manage the local response cache'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'waka;help;auth'= {
            cand login 'Log in with your `WakaTime` API key'
            cand logout 'Remove the stored API key'
            cand status 'Show whether you are currently logged in'
            cand show-key 'Display the stored API key (masked by default)'
            cand switch 'Switch to a different profile'
        }
        &'waka;help;auth;login'= {
        }
        &'waka;help;auth;logout'= {
        }
        &'waka;help;auth;status'= {
        }
        &'waka;help;auth;show-key'= {
        }
        &'waka;help;auth;switch'= {
        }
        &'waka;help;stats'= {
            cand today 'Show today''s coding activity'
            cand yesterday 'Show yesterday''s coding activity'
            cand week 'Show the last 7 days of activity'
            cand month 'Show the last 30 days of activity'
            cand year 'Show the last 365 days of activity'
            cand range 'Show activity for a custom date range'
        }
        &'waka;help;stats;today'= {
        }
        &'waka;help;stats;yesterday'= {
        }
        &'waka;help;stats;week'= {
        }
        &'waka;help;stats;month'= {
        }
        &'waka;help;stats;year'= {
        }
        &'waka;help;stats;range'= {
        }
        &'waka;help;projects'= {
            cand list 'List all projects with coding time'
            cand top 'Show the most active projects'
            cand show 'Show detailed stats for a project'
        }
        &'waka;help;projects;list'= {
        }
        &'waka;help;projects;top'= {
        }
        &'waka;help;projects;show'= {
        }
        &'waka;help;languages'= {
            cand list 'List all languages with coding time'
            cand top 'Show the top languages'
        }
        &'waka;help;languages;list'= {
        }
        &'waka;help;languages;top'= {
        }
        &'waka;help;editors'= {
            cand list 'List all editors with coding time'
            cand top 'Show the top editors'
        }
        &'waka;help;editors;list'= {
        }
        &'waka;help;editors;top'= {
        }
        &'waka;help;goals'= {
            cand list 'List all active goals'
            cand show 'Show details for a specific goal'
            cand watch 'Watch goals and refresh periodically'
        }
        &'waka;help;goals;list'= {
        }
        &'waka;help;goals;show'= {
        }
        &'waka;help;goals;watch'= {
        }
        &'waka;help;leaderboard'= {
            cand show 'Show the public leaderboard'
        }
        &'waka;help;leaderboard;show'= {
        }
        &'waka;help;report'= {
            cand generate 'Generate a productivity report for a date range'
            cand summary 'Show a brief productivity summary'
        }
        &'waka;help;report;generate'= {
        }
        &'waka;help;report;summary'= {
        }
        &'waka;help;dashboard'= {
        }
        &'waka;help;prompt'= {
        }
        &'waka;help;completions'= {
        }
        &'waka;help;config'= {
            cand get 'Get the value of a config key'
            cand set 'Set the value of a config key'
            cand edit 'Open the config file in $EDITOR'
            cand path 'Print the path to the config file'
            cand reset 'Reset config to defaults'
            cand doctor 'Run a full diagnostic check'
        }
        &'waka;help;config;get'= {
        }
        &'waka;help;config;set'= {
        }
        &'waka;help;config;edit'= {
        }
        &'waka;help;config;path'= {
        }
        &'waka;help;config;reset'= {
        }
        &'waka;help;config;doctor'= {
        }
        &'waka;help;cache'= {
            cand clear 'Clear all cached entries (or only those older than a duration)'
            cand info 'Show cache statistics (entry count, disk size, last write)'
            cand path 'Print the path to the cache directory'
        }
        &'waka;help;cache;clear'= {
        }
        &'waka;help;cache;info'= {
        }
        &'waka;help;cache;path'= {
        }
        &'waka;help;help'= {
        }
    ]
    $completions[$command]
}
