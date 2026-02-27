build:
    cargo build --release
    mkdir -p ~/.local/bin
    cp target/release/mb ~/.local/bin/
    @echo "Installed mb to ~/.local/bin/"
