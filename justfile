build:
    cargo build --release
    mkdir -p ~/.local/bin
    cp target/release/mb ~/.local/bin/
    @echo "Installed mb to ~/.local/bin/"

install-skill:
    rm -rf ~/.claude/skills/mb
    mkdir -p ~/.claude/skills/mb
    cp -r skills/mb/* ~/.claude/skills/mb/
    @echo "Skill installed to ~/.claude/skills/mb/"

install: build install-skill
