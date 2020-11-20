# i3title-rs

Prints the title of the focused i3 container. Meant as an i3-specific
replacement for xtitle that should work with i3blocks `persist=True`.

```
USAGE:
    i3title-rs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -s, --subscribe    Subscribe to i3 events. If set, i3title prints container titles line-by-line
                       as the focus changes
    -V, --version      Prints version information

OPTIONS:
    -t, --truncate <truncate>    Truncate the output to a length > 3. A value of zero means do not
                                 truncate [default: 0]
```
