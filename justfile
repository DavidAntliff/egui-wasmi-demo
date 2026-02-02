export RUST_LOG := 'info'

default:
    just --list

# Build everything
build: build-guest build-host

# Build the guest WASM application
build-guest:
    just -f guest/justfile build

# Build the host Wasmi application
build-host:
    just -f host/justfile build

# Build, upload and run the host Wasmi application
run: build-guest
    just -f host/justfile run

serve: build-guest
    just -f host/justfile serve

clean:
    just -f guest/justfile clean
    just -f host/justfile clean

ci:
    just -f host/justfile ci
    just -f guest/justfile ci
