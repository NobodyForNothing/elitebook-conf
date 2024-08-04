cargo build --release
cp ~/.profile ~/.profile.bak
echo "exec ~/.binprofile" > ~/.profile
cp target/release/binprofile ~/.binprofile