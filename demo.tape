Output demo.gif
Set Shell "bash"
Set Width 1200
Set Height 600
Set FontSize 14
Set Theme "Dracula"
Set Padding 20

Hide
Type "export PATH=/home/mouwaficbdr/Code/waka/target/release:$PATH"
Enter
Sleep 100ms
Type "waka() { /tmp/waka-demo $@; }"
Enter
Sleep 100ms
Type "clear"
Enter
Sleep 300ms
Show

Type "waka stats today"
Sleep 500ms
Enter
Sleep 5s
Enter

Sleep 2s

Type "waka stats week"
Sleep 500ms
Enter
Sleep 8s
Enter

Sleep 2s

Type "waka projects list"
Sleep 500ms
Enter
Sleep 6s
Enter

Sleep 3s

Type "waka dashboard"
Sleep 500ms
Enter
Sleep 8s
Type "q"
Sleep 2s