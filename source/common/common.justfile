COMMON_DIR := "${REPO_ROOT}/source/common"

# Copies the common vscode settings into the local project
[private]
ensure-vscode-directory:
    @echo "Linking local .vscode directory to common settings"
    @mkdir -p .vscode
    @rm -f .vscode/settings.json
    @ln -sf "{{COMMON_DIR}}/.vscode/settings.json" ".vscode/settings.json"
    @rm -f .vscode/extensions.json
    @ln -sf "{{COMMON_DIR}}/.vscode/extensions.json" ".vscode/extensions.json"
    @rm -f .vscode/fallgray-theme.json
    @ln -sf "{{COMMON_DIR}}/.vscode/fallgray-theme.json" ".vscode/fallgray-theme.json"

