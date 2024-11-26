run:
	@npm run tauri dev

test:
	@cd src-tauri && cargo test

fmt:
	@cd src-tauri && cargo fmt

clippy:
	@cd src-tauri && cargo clippy