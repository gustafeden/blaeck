Output demo_form.gif
Set Shell "bash"
Set FontSize 18
Set Width 650
Set Height 450
Set Padding 20

Hide
Type "cd /Users/gustafeden/gustaf/quill-fresh && cargo build --example demo_form 2>/dev/null"
Enter
Sleep 500ms
Type "clear"
Enter
Sleep 200ms
Show

Set TypingSpeed 0
Type "./target/debug/examples/demo_form"
Enter
Sleep 1000ms

# Type a project name
Set TypingSpeed 50ms
Type "my-app"
Sleep 500ms

# Tab to environment
Tab
Sleep 400ms

# Navigate environments
Down
Sleep 300ms
Down
Sleep 300ms
Up
Sleep 400ms

# Tab to confirm
Tab
Sleep 400ms

# Toggle checkbox
Type " "
Sleep 500ms

# Tab back to name
Tab
Sleep 400ms

# Add more text
Set TypingSpeed 50ms
Type "-v2"
Sleep 800ms

Ctrl+C
Sleep 300ms
