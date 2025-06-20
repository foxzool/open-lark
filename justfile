# Format code
fmt:
  @echo "🎨 Formatting code..."
  cargo fmt --all

# Check code formatting
fmt-check:
  @echo "🔍 Checking code format..."
  cargo fmt --all -- --check

# Lint code
lint:
  @echo "🔍 Linting code..."
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings

# Run tests
test:
  @echo "🧪 Running tests..."
  cargo test --all-features

# Build project
build:
  @echo "🔨 Building project..."
  cargo build --all-features

# Build release
build-release:
  @echo "🚀 Building release..."
  cargo build --release --all-features

# Generate documentation
docs:
  @echo "📚 Generating documentation..."
  cargo doc --all-features --no-deps

# Run all pre-release checks
check-all: fmt-check lint test build-release docs
  @echo "✅ All checks passed!"

# Release a new version
release VERSION:
  @echo "🚀 Starting release process for version {{VERSION}}"
  
  # Check if on main branch
  @if [ "$(git branch --show-current)" != "main" ]; then \
    echo "⚠️  Warning: You are not on the main branch (current: $(git branch --show-current))"; \
    read -p "Continue anyway? (y/N): " -n 1 -r; \
    echo; \
    if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
      echo "ℹ️  Aborting release"; \
      exit 1; \
    fi; \
  fi
  
  # Check if working directory is clean
  @if ! git diff-index --quiet HEAD --; then \
    echo "❌ Working directory is not clean. Please commit or stash your changes."; \
    git status --porcelain; \
    exit 1; \
  fi
  
  # Pull latest changes
  @echo "🔄 Pulling latest changes..."
  git pull origin main
  
  # Check if tag already exists
  @if git tag -l | grep -q "^v{{VERSION}}$$"; then \
    echo "❌ Tag v{{VERSION}} already exists"; \
    exit 1; \
  fi
  
  # Verify version in Cargo.toml
  @CARGO_VERSION=$$(grep '^version = ' Cargo.toml | cut -d'"' -f2); \
  if [ "$$CARGO_VERSION" != "{{VERSION}}" ]; then \
    echo "❌ Version mismatch: Cargo.toml has $$CARGO_VERSION, but you specified {{VERSION}}"; \
    echo "ℹ️  Please update Cargo.toml first"; \
    exit 1; \
  fi
  
  # Run pre-release checks
  @echo "🔍 Running pre-release checks..."
  just check-all
  
  # Check changelog
  @if ! grep -q "## \[{{VERSION}}\]" CHANGELOG.md; then \
    echo "⚠️  No changelog entry found for version {{VERSION}}"; \
    echo "ℹ️  Please update CHANGELOG.md before releasing"; \
    read -p "Continue anyway? (y/N): " -n 1 -r; \
    echo; \
    if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
      echo "ℹ️  Aborting release"; \
      exit 1; \
    fi; \
  fi
  
  # Create and push tag
  @echo "🏷️  Creating tag v{{VERSION}}..."
  git tag -a "v{{VERSION}}" -m "Release version {{VERSION}}"
  
  @echo "📤 Pushing tag to origin..."
  git push origin "v{{VERSION}}"
  
  @echo "✅ Tag v{{VERSION}} has been created and pushed!"
  @echo "ℹ️  GitHub Actions will now:"
  @echo "  1. Run validation tests"
  @echo "  2. Create a GitHub release"
  @echo "  3. Publish to crates.io"
  @echo ""
  @echo "ℹ️  Monitor progress at: https://github.com/foxzool/open-lark/actions"
  @echo ""
  @echo "🚀 Release process initiated successfully!"

# Show available commands
help:
  @echo "📋 Available commands:"
  @echo "  fmt          - Format code"
  @echo "  fmt-check    - Check code formatting"
  @echo "  lint         - Lint code with clippy"
  @echo "  test         - Run tests"
  @echo "  build        - Build project"
  @echo "  build-release - Build release version"
  @echo "  docs         - Generate documentation"
  @echo "  check-all    - Run all pre-release checks"
  @echo "  release VERSION - Release a new version (e.g., just release 0.4.0)"
  @echo "  help         - Show this help message"