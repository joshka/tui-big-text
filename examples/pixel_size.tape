# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/pixel_size.gif"
Set Theme "Aardvark Blue"
Set FontFamily "Free Mono"
Set Width 1520
Set Height 840
Hide
Type@0 "cargo run --example pixel_size --quiet"
Enter
Sleep 2s
Show
Screenshot "target/pixel_size.png"
Sleep 1s
