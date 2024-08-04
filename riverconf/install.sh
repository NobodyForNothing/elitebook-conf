cargo build --release
cp ~/.config/river/init ~/.config/river/init.bak
cp target/release/riverconf ~/.config/river/init
echo "> River configuration installed"
