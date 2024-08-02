# CURRENTLY NOT A SCRIPT
# handle more like an instruction on what general steps to do

# Setup files
echo "https://alpine.mirror.wearetriple.com/edge/main
https://alpine.mirror.wearetriple.com/edge/
https://alpine.mirror.wearetriple.com/edge/" > /etc/apk/repositories
apk update
apk upgrade

# Install river
setup-wayland-base
apk add mesa-dri-gallium mesa-va-gallium intel-media-driver
apk add river river-doc
setup-devd udev
apk add adwaita-icon-theme foot font-dejavu
install -Dm0755 /usr/share/doc/river/examples/init -t /home/rrausch/.config/river
chown rrausch -R /home/rrausch/.config/
# TODO: copy custom init as user
apk add pciutils-libs
apk add seatd
rc-update add seatd # configure it to auto-start
service seatd start # start it now
adduser rrausch seat # allow user to access seatd

# mako
apk add mako mako-doc mako-bash-completion

# Desktop software
apk add firefox thunar git gvfs mandoc
apk add build-base meson cairo cairo-dev pango-dev wayland-protocols wayland-protocols-dev libxkbcommon-dev

# TODO git setup:
# - ssh-keys
# - name and email

# dmenu-wayland
doas -u rrausch eval("\
mkdir /home/rrausch/apps\
cd /home/rrausch/apps\
git clone https://github.com/nyyManni/dmenu-wayland.git\
cd dmenu-wayland\
meson build\
ninja -C build")
cd /home/rrausch/apps/dmenu-wayland
ninja -C build install
