@echo OFF
cargo doc -p float_eq --all-features --no-deps
mdbook build book -d ../target/book %*