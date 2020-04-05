@echo off
cargo test --quiet


echo Generating README...
cargo readme --no-license > crates-io.md
cargo readme --no-license > README.md
echo. >> README.md
type LICENSE.md >> README.md

echo.
echo Packaging...
cargo package

echo.
echo If all went well, `cargo publish`!