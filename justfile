bevy_version := "v0.18.1"
bevy_egui_version := "v0.39.1"
bevy_repo := "https://github.com/bevyengine/bevy.git"
bevy_egui_repo := "https://github.com/mvlabat/bevy_egui.git"
clone_bevy := "bevy_tmp"
clone_egui := "bevy_egui_tmp"

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
