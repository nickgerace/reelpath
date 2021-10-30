MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

all:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo clippy --all-features --all-targets
	cd $(MAKEPATH); cargo build --release
