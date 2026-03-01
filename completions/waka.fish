# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_waka_global_optspecs
	string join \n p/profile= f/format= no-cache no-color quiet verbose csv-bom h/help V/version
end

function __fish_waka_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_waka_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_waka_using_subcommand
	set -l cmd (__fish_waka_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c waka -n "__fish_waka_needs_command" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_needs_command" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_needs_command" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_needs_command" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_needs_command" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_needs_command" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_needs_command" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_needs_command" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_needs_command" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_needs_command" -f -a "auth" -d 'Manage API key and authentication'
complete -c waka -n "__fish_waka_needs_command" -f -a "stats" -d 'Show coding statistics'
complete -c waka -n "__fish_waka_needs_command" -f -a "projects" -d 'Browse and filter projects'
complete -c waka -n "__fish_waka_needs_command" -f -a "languages" -d 'Browse coding languages'
complete -c waka -n "__fish_waka_needs_command" -f -a "editors" -d 'Browse editors and IDEs'
complete -c waka -n "__fish_waka_needs_command" -f -a "goals" -d 'View and watch coding goals'
complete -c waka -n "__fish_waka_needs_command" -f -a "leaderboard" -d 'View the `WakaTime` leaderboard'
complete -c waka -n "__fish_waka_needs_command" -f -a "report" -d 'Generate productivity reports'
complete -c waka -n "__fish_waka_needs_command" -f -a "dashboard" -d 'Launch the interactive TUI dashboard'
complete -c waka -n "__fish_waka_needs_command" -f -a "prompt" -d 'Shell prompt integration (reads from cache only, no network)'
complete -c waka -n "__fish_waka_needs_command" -f -a "completions" -d 'Generate shell completions'
complete -c waka -n "__fish_waka_needs_command" -f -a "config" -d 'Manage waka configuration'
complete -c waka -n "__fish_waka_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "login" -d 'Log in with your `WakaTime` API key'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "logout" -d 'Remove the stored API key'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "status" -d 'Show whether you are currently logged in'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "show-key" -d 'Display the stored API key (masked by default)'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "switch" -d 'Switch to a different profile'
complete -c waka -n "__fish_waka_using_subcommand auth; and not __fish_seen_subcommand_from login logout status show-key switch help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l api-key -d 'Provide the API key directly (non-interactive)' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l profile -d 'Profile to store credentials under' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from login" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l profile -d 'Log out a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from logout" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from status" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from show-key" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from switch" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "login" -d 'Log in with your `WakaTime` API key'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "logout" -d 'Remove the stored API key'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "status" -d 'Show whether you are currently logged in'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "show-key" -d 'Display the stored API key (masked by default)'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "switch" -d 'Switch to a different profile'
complete -c waka -n "__fish_waka_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "today" -d 'Show today\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "yesterday" -d 'Show yesterday\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "week" -d 'Show the last 7 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "month" -d 'Show the last 30 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "year" -d 'Show the last 365 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "range" -d 'Show activity for a custom date range'
complete -c waka -n "__fish_waka_using_subcommand stats; and not __fish_seen_subcommand_from today yesterday week month year range help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from today" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from yesterday" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from week" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from month" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from year" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l from -d 'Start date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l to -d 'End date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l project -d 'Filter by project name' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l language -d 'Filter by language' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from range" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "today" -d 'Show today\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "yesterday" -d 'Show yesterday\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "week" -d 'Show the last 7 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "month" -d 'Show the last 30 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "year" -d 'Show the last 365 days of activity'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "range" -d 'Show activity for a custom date range'
complete -c waka -n "__fish_waka_using_subcommand stats; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -f -a "list" -d 'List all projects with coding time'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -f -a "top" -d 'Show the most active projects'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -f -a "show" -d 'Show detailed stats for a project'
complete -c waka -n "__fish_waka_using_subcommand projects; and not __fish_seen_subcommand_from list top show help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l sort-by -d 'Sort field' -r -f -a "time\t''
name\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l limit -d 'Maximum number of results' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l period -d 'Time period to aggregate over' -r -f -a "7d\t''
30d\t''
1y\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from top" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l from -d 'Start date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l to -d 'End date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from show" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all projects with coding time'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from help" -f -a "top" -d 'Show the most active projects'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show detailed stats for a project'
complete -c waka -n "__fish_waka_using_subcommand projects; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -f -a "list" -d 'List all languages with coding time'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -f -a "top" -d 'Show the top languages'
complete -c waka -n "__fish_waka_using_subcommand languages; and not __fish_seen_subcommand_from list top help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l period -d 'Time period to aggregate over' -r -f -a "7d\t''
30d\t''
1y\t''"
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l limit -d 'Maximum number of results' -r
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from top" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all languages with coding time'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from help" -f -a "top" -d 'Show the top languages'
complete -c waka -n "__fish_waka_using_subcommand languages; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -f -a "list" -d 'List all editors with coding time'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -f -a "top" -d 'Show the top editors'
complete -c waka -n "__fish_waka_using_subcommand editors; and not __fish_seen_subcommand_from list top help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l period -d 'Time period to aggregate over' -r -f -a "7d\t''
30d\t''
1y\t''"
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l limit -d 'Maximum number of results' -r
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from top" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all editors with coding time'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from help" -f -a "top" -d 'Show the top editors'
complete -c waka -n "__fish_waka_using_subcommand editors; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -f -a "list" -d 'List all active goals'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -f -a "show" -d 'Show details for a specific goal'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -f -a "watch" -d 'Watch goals and refresh periodically'
complete -c waka -n "__fish_waka_using_subcommand goals; and not __fish_seen_subcommand_from list show watch help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from show" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l interval -d 'Refresh interval in seconds' -r
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l notify -d 'Send a desktop notification when a goal is reached'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from watch" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all active goals'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show details for a specific goal'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from help" -f -a "watch" -d 'Watch goals and refresh periodically'
complete -c waka -n "__fish_waka_using_subcommand goals; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -f -a "show" -d 'Show the public leaderboard'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and not __fish_seen_subcommand_from show help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l page -d 'Page number' -r
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from show" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show the public leaderboard'
complete -c waka -n "__fish_waka_using_subcommand leaderboard; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -f -a "generate" -d 'Generate a productivity report for a date range'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -f -a "summary" -d 'Show a brief productivity summary'
complete -c waka -n "__fish_waka_using_subcommand report; and not __fish_seen_subcommand_from generate summary help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l from -d 'Start date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l to -d 'End date (YYYY-MM-DD)' -r
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -s o -l output -d 'Output file path' -r -F
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -s f -l format -d 'Report format' -r -f -a "md\t''
html\t''
json\t''
csv\t''"
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from generate" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l period -d 'Period to summarise' -r -f -a "week\t''
month\t''"
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from summary" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from help" -f -a "generate" -d 'Generate a productivity report for a date range'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from help" -f -a "summary" -d 'Show a brief productivity summary'
complete -c waka -n "__fish_waka_using_subcommand report; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l refresh -d 'Auto-refresh interval in seconds' -r
complete -c waka -n "__fish_waka_using_subcommand dashboard" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand dashboard" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand dashboard" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand prompt" -l format -d 'Output style' -r -f -a "simple\t''
detailed\t''"
complete -c waka -n "__fish_waka_using_subcommand prompt" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand prompt" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand prompt" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand prompt" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand prompt" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand prompt" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand prompt" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand prompt" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand completions" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand completions" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand completions" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand completions" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand completions" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand completions" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand completions" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand completions" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c waka -n "__fish_waka_using_subcommand completions" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "get" -d 'Get the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "set" -d 'Set the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "edit" -d 'Open the config file in $EDITOR'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "path" -d 'Print the path to the config file'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "reset" -d 'Reset config to defaults'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "doctor" -d 'Run a full diagnostic check'
complete -c waka -n "__fish_waka_using_subcommand config; and not __fish_seen_subcommand_from get set edit path reset doctor help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from get" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from set" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from edit" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from path" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l confirm -d 'Skip the confirmation prompt'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from reset" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -s p -l profile -d 'Use a specific profile' -r
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -s f -l format -d 'Output format: table, json, csv, plain' -r -f -a "table\t''
json\t''
csv\t''
plain\t''"
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -l no-cache -d 'Skip the cache and force a fresh API request'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -l no-color -d 'Disable colors (equivalent to `NO_COLOR=1`)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -l quiet -d 'Suppress non-essential output'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -l verbose -d 'Enable verbose mode (shows HTTP requests)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -l csv-bom -d 'Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility)'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -s h -l help -d 'Print help'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from doctor" -s V -l version -d 'Print version'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "get" -d 'Get the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "set" -d 'Set the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Open the config file in $EDITOR'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "path" -d 'Print the path to the config file'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "reset" -d 'Reset config to defaults'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "doctor" -d 'Run a full diagnostic check'
complete -c waka -n "__fish_waka_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "auth" -d 'Manage API key and authentication'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "stats" -d 'Show coding statistics'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "projects" -d 'Browse and filter projects'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "languages" -d 'Browse coding languages'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "editors" -d 'Browse editors and IDEs'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "goals" -d 'View and watch coding goals'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "leaderboard" -d 'View the `WakaTime` leaderboard'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "report" -d 'Generate productivity reports'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "dashboard" -d 'Launch the interactive TUI dashboard'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "prompt" -d 'Shell prompt integration (reads from cache only, no network)'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "completions" -d 'Generate shell completions'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "config" -d 'Manage waka configuration'
complete -c waka -n "__fish_waka_using_subcommand help; and not __fish_seen_subcommand_from auth stats projects languages editors goals leaderboard report dashboard prompt completions config help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "login" -d 'Log in with your `WakaTime` API key'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "logout" -d 'Remove the stored API key'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "status" -d 'Show whether you are currently logged in'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "show-key" -d 'Display the stored API key (masked by default)'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "switch" -d 'Switch to a different profile'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "today" -d 'Show today\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "yesterday" -d 'Show yesterday\'s coding activity'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "week" -d 'Show the last 7 days of activity'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "month" -d 'Show the last 30 days of activity'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "year" -d 'Show the last 365 days of activity'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from stats" -f -a "range" -d 'Show activity for a custom date range'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from projects" -f -a "list" -d 'List all projects with coding time'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from projects" -f -a "top" -d 'Show the most active projects'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from projects" -f -a "show" -d 'Show detailed stats for a project'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from languages" -f -a "list" -d 'List all languages with coding time'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from languages" -f -a "top" -d 'Show the top languages'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from editors" -f -a "list" -d 'List all editors with coding time'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from editors" -f -a "top" -d 'Show the top editors'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from goals" -f -a "list" -d 'List all active goals'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from goals" -f -a "show" -d 'Show details for a specific goal'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from goals" -f -a "watch" -d 'Watch goals and refresh periodically'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from leaderboard" -f -a "show" -d 'Show the public leaderboard'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from report" -f -a "generate" -d 'Generate a productivity report for a date range'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from report" -f -a "summary" -d 'Show a brief productivity summary'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "get" -d 'Get the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "set" -d 'Set the value of a config key'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "edit" -d 'Open the config file in $EDITOR'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "path" -d 'Print the path to the config file'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "reset" -d 'Reset config to defaults'
complete -c waka -n "__fish_waka_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "doctor" -d 'Run a full diagnostic check'
