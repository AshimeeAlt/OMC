# OMC

OMC Stands for `Origin Machine Code` its a low level language for [OriginOS](https://github.com/Mistium/Origin-OS/) by [@Mistium](https://github.com/Mistium/)

### Features

It has all the same features as [OASM](https://github.com/Mistium/Origin-OS/wiki/OASM-%E2%80%90-Origin-Assembly/) but its smaller and faster to parse

There are some special functions like `svtf` for hard to parse types (see "Custom Commands")

### Output options

You can currently convert your `OCM` to [OASM](https://github.com/Mistium/Origin-OS/wiki/OASM-%E2%80%90-Origin-Assembly/) and vice versa.

I plan on adding OTAS support later on but right now thats a bit to hard to parse with my skill level (for reference I just started using rust when I made this!), along with converting your [OASM](https://github.com/Mistium/Origin-OS/wiki/OASM-%E2%80%90-Origin-Assembly/) or `OCM` to raw JS functions to run in the `VM` (this is just to complicated to make with the current system it was started).

#### Custom Commands

(These are intermediary compiler commands so you can use the in OMC but not OASM)

<br />

`svtf`: sets a variable to a float.

(takes 3 arguments `var` `base` and `decimal` and is converted to `setv {var} {base}.{decimal}`)

<br />

`svts`: sets a variable to a string.

(takes 2 arguments `var` `string length`)

this will take the next `string length` bytes and set that as the variable

(and is converted to `setv {var} {string}`)

#### Legal

`OMC` is not affiliated with `OriginOS`, `Origin`, `OASM`, `OTAS` or `Mistium`, nor is `OMC` endorsed by any of the above, This is purely a fan project.

The **license** is `LGPLv3 w/ MIT`
