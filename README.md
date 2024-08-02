# Notebook setup 

Linux desktop software and configs that run on a normal alpine linux install. 
Target user account: rrausch

After installing all software manually install.sh can be run to copy configurations and custom software.

## Software

- river compositor
- foot
- dmenu-wayland
- mako

### [Autologin](https://wiki.alpinelinux.org/wiki/TTY_Autologin)

To enable autologin copy the `autologin` file to `/usr/sbin` and in `/etc/inittab` replace ":respawn:/sbin/getty" with ":respawn:/sbin/getty -n -l /usr/sbin/autologin" for each TTY you want to enable autologin.

## TODO

- .profile
- configure brightnessctl to not require root
- notification:
- some bar: https://codeberg.org/river/wiki/src/branch/master/pages/Recommended-Software.md
- https://codeberg.org/ifreund/waylock