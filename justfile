#!/bin/bash

fmt:
	cargo fmt --all --check

lint:
	cargo clippy --fix --tests -- -D warnings
   
test:
    cargo test

wasm:
	sh scripts/sh/optimize.sh

schema:
	sh scripts/sh/schema-and-codegen.sh

codegen:
	cd scripts/ts/ && npm run codegen

# publish:
#     #!/usr/bin/env bash
#     crates=()
#     for crate in "${crates[@]}"; do
#       cargo publish -p "$crate"
#       echo "Sleeping before publishing the next crate..."
#       sleep 30
#     done

