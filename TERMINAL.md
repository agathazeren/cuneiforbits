# Terminal Compatability with Cuneiforbits

The terminal must be able to render a non-monospace font, and a font that supports cuneiform script must be installed and loaded into the terminal.

## Fonts

It is recommended to use the included Cuneiforbits font, as it has monospace latin characters, in addition to having the cuneiform characters availible, but any font with these properties will work. 

### Cuneforbits Font

This font is a merge of Google's Noto Sans Cuneiform into Noto Sans Mono. Included is a .sfd, the native file format of FontForge. In order to use this program, open Cuneiforbits.sfd in FontForge, and then use File>Generate Fonts to emit a ttf file. Then install the font on your system. 

## Terminals

Not all terminals work. I have only gotten it to work on Alacritty. See below for configureation

### Alacritty

You will need to set up an alacritty.yml, in one of the various places it searches. Then, in `font:normal:family`, name a font. 
