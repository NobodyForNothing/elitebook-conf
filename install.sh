# build binaries
cd riverconf || (echo "wrong cwd" && exit)
cargo build --release || exit
cd ..

# copy files
cp --backup dotprofile ~/.profile # fixme: alpine cp doesn't have --backup => create rust program
cp riverconf/target/release/riverconf ~/.config/river/init
cp -r mako ~/.config/