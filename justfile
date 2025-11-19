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
  @echo "🔍 Linting code (exclude benches/dev-tests)..."
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

# Run coverage tests (requires cargo-llvm-cov)
coverage:
  @echo "📊 Running coverage analysis..."
  cargo llvm-cov clean --workspace
  cargo llvm-cov test --all-features --workspace \
    --html --output-dir target/llvm-cov/html
  cargo llvm-cov report --lcov --output-path target/llvm-cov/lcov.info
  @echo "📋 Coverage report generated:"
  @echo "  - HTML: target/llvm-cov/html/index.html"
  @echo "  - LCOV: target/llvm-cov/lcov.info"

# Run coverage with threshold check
coverage-check:
  @echo "📊 Running coverage with threshold check..."
  just coverage
  @echo "🔍 Checking coverage threshold..."
  @cargo llvm-cov report --summary-only | tail -1 | awk '{ \
    gsub(/%/, "", $$7); \
    cov = $$7; \
    printf "📊 Coverage: %s%%\n", cov; \
    min = "${MIN_COVERAGE:-54.0}"; \
    if (cov+0 >= min+0) { \
      print "✅ Coverage " cov "% >= threshold " min "%"; \
    } else { \
      print "❌ Coverage " cov "% < threshold " min "%"; \
      exit 1; \
    } \
  }'

# Run security audit
audit:
  @echo "🔒 Running security audit..."
  @echo "Checking for security vulnerabilities..."
  cargo audit
  @echo "Checking licenses and supply chain..."
  cargo deny check
  @echo "✅ Security audit completed!"

# Update security databases
update-audit-db:
  @echo "🔄 Updating security advisory database..."
  cargo audit --update
  @echo "✅ Security database updated!"

# Test feature combinations (requires cargo-hack)
test-features:
  @echo "🧪 Testing feature combinations..."
  @echo "Testing each feature individually (excluding heavy features)..."
  cargo hack test --each-feature --exclude-features websocket --lib
  @echo "Testing core feature combinations..."
  cargo hack test --feature-powerset --depth 2 \
    --features "im,cloud-docs,contact,group,authentication,search" --lib
  @echo "✅ Feature matrix testing completed!"

# Quick feature combination test (most common combinations)
test-features-quick:
  @echo "🧪 Quick feature combination testing..."
  @echo "Testing no features..."
  cargo test --no-default-features --lib
  @echo "Testing default features..."
  cargo test --lib
  @echo "Testing all features..."
  cargo test --all-features --lib
  @echo "Testing websocket feature..."
  cargo test --no-default-features --features websocket --lib
  @echo "✅ Quick feature testing completed!"

# Install development tools
install-dev-tools:
  @echo "🛠️ Installing development tools..."
  cargo install cargo-llvm-cov cargo-audit cargo-deny cargo-hack
  @echo "✅ Development tools installed!"

# Run all pre-release checks including coverage and security
check-all: fmt-check lint test coverage-check audit build-release docs
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
  @echo "  test-features - Test all feature combinations (slow)"
  @echo "  test-features-quick - Test common feature combinations"
  @echo "  build        - Build project"
  @echo "  build-release - Build release version"
  @echo "  docs         - Generate documentation"
  @echo "  coverage     - Run coverage analysis"
  @echo "  coverage-check - Run coverage with threshold check"
  @echo "  audit        - Run security audit"
  @echo "  update-audit-db - Update security advisory database"
  @echo "  install-dev-tools - Install development tools"
  @echo "  check-all    - Run all pre-release checks (includes coverage & security)"
  @echo "  release VERSION - Release a new version (e.g., just release 0.4.0)"
  @echo "  help         - Show this help message"
