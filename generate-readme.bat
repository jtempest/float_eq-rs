@echo off

echo ^<!--> README.md
echo     THIS FILE IS GENERATED FROM crates-io.md AND LICENSE.md.>> README.md
echo     DO NOT EDIT IT DIRECTLY.>> README.md
echo --^>>> README.md
echo.>> README.md
type "float_eq\crates-io.md" >> README.md
echo.>> README.md
echo ^<br^>>> README.md
echo.>> README.md
type LICENSE.md >> README.md
