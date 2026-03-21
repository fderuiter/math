echo "Testing math_explorer"
cargo test -p math_explorer
echo "Testing clippy"
cargo clippy -p math_explorer -- -D warnings
