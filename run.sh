echo "Starting bank app"
cargo build
./target/debug/banking_app < commands.txt
echo "Bank app finished." 