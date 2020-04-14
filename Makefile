install:
	cargo build --release
	sudo mkdir -p "${HOME}/.snyfy/bin"
	sudo cp "./target/release/snyfy" "${HOME}/.snyfy/bin"
	sudo chmod +x "${HOME}/.snyfy/bin/snyfy"
	@echo ""
	@echo "Add the the CLI to your path with:"
	@echo ""
	@echo "  export PATH=\$$PATH:${HOME}/.snyfy/bin"
	@echo ""
	@echo "Looking for more? Visit https://github.com/mayankshah1607/snyfy"
	@echo ""
	exit 0
