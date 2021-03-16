MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
VERSION:=$(shell cd $(MAKEPATH) && cargo metadata -q | jq -r '.packages[0].version')

all: build

build: pre-build
	cd $(MAKEPATH); cargo build

build-release: pre-build doc
	cd $(MAKEPATH); cargo build --release

pre-build:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fmt
	cd $(MAKEPATH); cargo clippy

doc:
	cd $(MAKEPATH); cargo doc --open

release-tag:
	cd $(MAKEPATH); git tag $(VERSION)
	cd $(MAKEPATH); git push --tags origin main
	cd $(MAKEPATH); cargo publish

release-prepare:
	@printf "[1] Change version in Cargo.toml: $(VERSION)\n"
	@printf "[2] Then, run the following command...\n"
	@printf "    time make build-release\n"
	@printf "[3] Create a commit with the following message...\n"
	@printf "    Update to x.x.x\n"
	@printf "[4] Before merging, ensure that publishing works.\n"
	@printf "    cargo publish --dry-run\n"

release-post-tag:
	@printf "[1] Run the following command...\n"
	@printf "    time make release-tag\n"
	@printf "[2] Edit the GitHub release page for the new release.\n"
	@printf "[3] Check crates.io: https://crates.io/crates/reelpath\n"
	@printf "[4] Update Homebrew tap version: https://github.com/nickgerace/homebrew-reelpath\n"

