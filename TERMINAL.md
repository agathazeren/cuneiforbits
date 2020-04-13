# Terminal Compatability with Cuneiforbits

The terminal must be able to render a non-monospace font, and a font that supports cuneiform script must be installed and loaded into the terminal.

## Fonts

One font that does support cuneiform characters is Noto Sans Cuneiform. Others may be availible. 

## Terminals

Not all terminals work. I have only gotten it to work on Alacritty. See below for configureation

### Alacritty

You will need to set up a alacritty.yml, in one of the various places it searches. Then, in `font:normal:family`, add the fonts and fall backs that you desire, makeing sure that a cuneifrom 
supporting font is in the list. You may also need to run alacritty with the `-v` option in order to reload the configureation. 


