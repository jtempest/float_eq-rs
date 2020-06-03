@echo off
REM cargo hack test --feature-powerset --lib -q
cargo hack test --feature-powerset --tests -q
cargo test --all-features --doc

cargo doc --all-features

echo Generating README...
echo ^<!--> README.md
echo     THIS FILE IS GENERATED FROM crates-io.md AND LICENSE.md.>> README.md
echo     DO NOT EDIT IT DIRECTLY.>> README.md
echo --^>>> README.md
echo.>> README.md
type crates-io.md >> README.md
echo.>> README.md
echo ^<br^>>> README.md
echo.>> README.md
type LICENSE.md >> README.md

echo.
echo Packaging...
cargo package

echo.
echo If all went well, `cargo publish`!