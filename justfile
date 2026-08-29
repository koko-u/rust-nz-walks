workspace_root := justfile_directory()

_default:
    @just --list

# run app
run:
    @cargo run --package api --bin server --features api-doc

# watch app
watch:
    @cargo watch --clear --quiet --exec "run --package api --bin server --features api-doc"

# docker up
dup:
    @docker compose -f {{ workspace_root }}/docker/compose.yml up -d

# docker down
dwn:
    @docker compose -f {{ workspace_root }}/docker/compose.yml down

# format
fmt:
    @cargo +nightly fmt
