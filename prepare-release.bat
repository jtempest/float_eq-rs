@echo off
cargo test --quiet
cargo doc
cargo test --quiet --no-default-features
cargo doc --no-default-features

echo.
echo Packaging...
cargo package

echo.
echo If all went well, `cargo publish`!