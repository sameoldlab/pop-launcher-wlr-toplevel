#!/usr/bin/env just --justfile

install:
    cargo build --release
    mkdir -p ~/.local/share/pop-launcher/plugins/toplevel
    install -Dm0755 target/release/toplevel ~/.local/share/pop-launcher/plugins/toplevel/toplevel
    install -Dm644 plugin.ron ~/.local/share/pop-launcher/plugins/toplevel/plugin.ron
    # sudo install -Dm644 icon.svg /usr/share/pixmaps/toplevel.svg
