# build binaries
cd riverconf || (echo "wrong cwd" && exit)
cargo build --release || exit
cd ..

# copy files
cp --backup dotprofile ~/.profile
cp riverconf/target/riverconf ~/.config/river/init
cp -r mako ~/.config/