@echo off
cargo test --quiet


echo Generating README...
type crates-io.md > README.md
echo ^<br^>
echo. >> README.md
type LICENSE.md >> README.md

echo.
echo Packaging...
cargo package

echo.
echo If all went well, `cargo publish`!