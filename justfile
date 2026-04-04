bevy_version := "v0.18.1"
bevy_egui_version := "v0.39.1"
bevy_repo := "https://github.com/bevyengine/bevy.git"
bevy_egui_repo := "https://github.com/mvlabat/bevy_egui.git"
clone_bevy := "bevy_tmp"
clone_egui := "bevy_egui_tmp"
PROJECT_ROOT := justfile_directory()

# setup ===============================================================================================================
dump_ex:
    `@set` -e; \
    trap 'rm -rf {{ clone_bevy }} {{ clone_egui }}' EXIT; \
    echo "==> Cloning Bevy {{ bevy_version }}..."; \
    git clone --depth 1 -b {{ bevy_version }} {{ bevy_repo }} {{ clone_bevy }}; \
    echo "==> Generating Bevy documentation..."; \
    uv run tools/dump_example.py {{ clone_bevy }}/examples bevy_{{ bevy_version }}_examples.md "Bevy {{ bevy_version }}"; \
    echo "==> Cloning bevy_egui {{ bevy_egui_version }}..."; \
    git clone --depth 1 -b {{ bevy_egui_version }} {{ bevy_egui_repo }} {{ clone_egui }}; \
    echo "==> Generating bevy_egui documentation..."; \
    uv run tools/dump_example.py {{ clone_egui }}/examples bevy_egui_{{ bevy_egui_version }}_examples.md "bevy_egui {{ bevy_egui_version }}"; \
    echo "==> Done! Both markdown files have been generated."

# dev ===============================================================================================================
fix-rs:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --workspace -- -D warnings

fmt-rs:
    just fix-rs
    cargo fmt --all

fmt: fmt-rs

test-rs:
    cargo test --workspace

test: test-rs

# run ===============================================================================================================

# setup example (see also Dockerfile):
#  brew install trunk
#  cargo install wasm-server-runner
#  rustup target add wasm32-unknown-unknown
run-wasm:
    trunk serve

run-docker-compose-up-build:
    docker compose up --build

# houdini ============================================================================================================
HOUDINI_VEX_PATH := PROJECT_ROOT + "/vex/include"
# Override via HOUDINI_RESOURCES env var for your platform/version
HOUDINI_RESOURCES := env_var_or_default("HOUDINI_RESOURCES", "/Applications/Houdini/Houdini21.0.631/Frameworks/Houdini.framework/Versions/Current/Resources")
# This env var should be set for untrusted localhost.
HOUDINI_RAMEN_TOKEN := env_var_or_default("HOUDINI_RAMEN_TOKEN", "houdini_ramen_secret_2026")
HOUDINI_RAMEN_PORT := env_var_or_default("HOUDINI_RAMEN_PORT", "18080")

houdini-link:
    HOUDINI_VEX_PATH="{{ HOUDINI_VEX_PATH }};&" HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} HOUDINI_RAMEN_PORT={{ HOUDINI_RAMEN_PORT }} {{ HOUDINI_RESOURCES }}/bin/houdini ramen_assets/link_server.py

run-live:
    HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} cargo run -p ramen_assets

# python ==========================================================
fmt-py:
    uv run ruff format tools ramen_assets

# misc ===========================================================================
pch:
    set -e
    gitleaks protect --staged --redact --no-banner
    ! rg '[\p{Han}\p{Hiragana}\p{Katakana}]' src crates assets ramen_assets .cargo .github tools
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
