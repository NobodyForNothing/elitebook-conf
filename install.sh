# build binaries
cd riverconf || (echo "wrong cwd" && exit)
./install.sh
cd ../binprofile
./install.sh
cd ..

# copy files
cp -r mako ~/.config/