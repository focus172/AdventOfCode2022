default:
    @just -l

publish:
    cargo fmt
    cargo clippy
