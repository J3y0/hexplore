# build in debug mode
debug:
    @echo "building for debug.."
    cargo build

# build in release mode
release:
    @echo "building for release.."
    cargo build --release

# check with clippy
clippy:
    @echo "running clippy.."
    cargo clippy

# lint with cargo fmt
lint:
    @echo "formatting repository.."
    cargo fmt --all --check

# check typos with typos-cli
typos:
    @echo "checking typos.."
    typos --config typos.toml

# apply typos-cli fix
typos-fix:
    @echo "fixing typos.."
    typos --config typos.toml -w

# cargo clean
clean:
    @echo "cleaning..."
    cargo clean

# command running checks before committing
check: lint typos clippy
