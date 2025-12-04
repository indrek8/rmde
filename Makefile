.PHONY: all build-rust build-swift clean run xcode

# Build everything
all: build-rust copy-bridge

# Generate Xcode project (requires: brew install xcodegen)
xcode: copy-bridge
	cd RMDE && xcodegen generate
	@echo "Xcode project generated at RMDE/RMDE.xcodeproj"
	@echo "Open with: open RMDE/RMDE.xcodeproj"

# Build Rust library
build-rust:
	cargo build --release

# Copy generated Swift bridge files to RMDE app
copy-bridge: build-rust
	cp rmde-core/generated/SwiftBridgeCore.swift RMDE/Sources/Bridge/
	cp rmde-core/generated/rmde-core/rmde-core.swift RMDE/Sources/Bridge/RMDECore.swift
	@echo "Bridge files copied to RMDE/Sources/Bridge/"

# Build for Apple Silicon (arm64)
build-macos-arm64:
	cargo build --release --target aarch64-apple-darwin
	@echo "Static library at: target/aarch64-apple-darwin/release/librmde_core.a"

# Build for Intel (x86_64)
build-macos-x86:
	cargo build --release --target x86_64-apple-darwin
	@echo "Static library at: target/x86_64-apple-darwin/release/librmde_core.a"

# Build universal binary
build-macos-universal: build-macos-arm64 build-macos-x86
	mkdir -p target/universal/release
	lipo -create \
		target/aarch64-apple-darwin/release/librmde_core.a \
		target/x86_64-apple-darwin/release/librmde_core.a \
		-output target/universal/release/librmde_core.a
	@echo "Universal library at: target/universal/release/librmde_core.a"

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf RMDE/Sources/Bridge/SwiftBridgeCore.swift
	rm -rf RMDE/Sources/Bridge/RMDECore.swift

# Show project structure
tree:
	@find . -type f -name "*.rs" -o -name "*.swift" -o -name "*.toml" | grep -v target | grep -v .build | head -50
