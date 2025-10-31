import "source/common/common.justfile"

[private]
default:
    @just --list --unsorted

ensure:
    @just ensure-vscode-directory
    
# Runs the demo project
demo:
    echo "TODO"

# Builds all projects
build: ensure
    cd source/assets && just build
    cd source/cmd/fallgray && just build
    cd source/cmd/snowfall && just build

# Tests all projects
test: build
    cd source/modules/raiment-core && just test
       
# Publishes all projects 
publish:
    echo "TODO"

# Restores the repository to a clean state
clean:
    git clean -fdx
    find . -type d -empty -delete

[private]
clean-bin:
    cd bin && git clean -fdx
