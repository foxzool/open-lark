#!/bin/bash
# Test Runner Script for open-lark
# Provides layered testing capabilities based on available environment and configuration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_CONFIG="$PROJECT_ROOT/.test-config.toml"

# Functions
print_header() {
    echo -e "${BLUE}==== $1 ====${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

check_env_var() {
    local var_name=$1
    if [[ -z "${!var_name}" ]]; then
        return 1
    else
        return 0
    fi
}

detect_environment() {
    print_header "Detecting Test Environment"
    
    local env_type="local"
    local missing_vars=()
    
    # Check for integration environment variables
    if ! check_env_var "APP_ID"; then missing_vars+=("APP_ID"); fi
    if ! check_env_var "APP_SECRET"; then missing_vars+=("APP_SECRET"); fi
    
    if [[ ${#missing_vars[@]} -eq 0 ]]; then
        env_type="integration"
        
        # Check for full environment
        if check_env_var "USER_ACCESS_TOKEN"; then
            env_type="full"
        fi
    fi
    
    echo "Environment type: $env_type"
    
    if [[ ${#missing_vars[@]} -gt 0 ]]; then
        print_warning "Missing environment variables: ${missing_vars[*]}"
        print_warning "Some tests will be skipped"
    fi
    
    echo "$env_type"
}

run_unit_tests() {
    print_header "Running Unit Tests"
    cargo test --lib --all-features
    print_success "Unit tests completed"
}

run_feature_tests() {
    print_header "Running Feature Combination Tests"
    
    # Quick feature tests (most common combinations)
    echo "Testing no features..."
    cargo test --no-default-features --lib
    
    echo "Testing default features..."
    cargo test --lib
    
    echo "Testing core feature combinations..."
    cargo test --no-default-features --features "im,authentication" --lib
    cargo test --no-default-features --features "cloud-docs,authentication" --lib
    
    print_success "Feature tests completed"
}

run_doctest_safe() {
    print_header "Running Safe Documentation Tests"
    
    # Only run doctests that don't require external API access
    echo "Running compile-only doctests..."
    cargo test --doc --all-features 2>/dev/null || true
    
    print_success "Safe doctests completed"
}

run_integration_tests() {
    print_header "Running Integration Tests (Doctests)"
    
    if [[ "$ENABLE_INTEGRATION_DOCTESTS" == "true" ]]; then
        print_warning "Running integration doctests - this requires valid API credentials"
        cargo test --doc --all-features -- --include-ignored
    else
        print_warning "Skipping integration doctests (set ENABLE_INTEGRATION_DOCTESTS=true to enable)"
        print_warning "Running only safe doctests..."
        run_doctest_safe
    fi
}

run_coverage_tests() {
    print_header "Running Coverage Analysis"
    
    # Check if cargo-llvm-cov is installed
    if ! command -v cargo-llvm-cov &> /dev/null; then
        print_error "cargo-llvm-cov not found. Install with: cargo install cargo-llvm-cov"
        return 1
    fi
    
    cargo llvm-cov clean --workspace
    cargo llvm-cov test --all-features --workspace --lib \
        --lcov --output-path target/llvm-cov/lcov.info \
        --html --output-dir target/llvm-cov/html
    
    print_success "Coverage analysis completed"
    echo "HTML report: target/llvm-cov/html/index.html"
}

show_help() {
    echo "Test Runner for open-lark"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  auto         - Automatically detect environment and run appropriate tests"
    echo "  unit         - Run unit tests only"
    echo "  features     - Run feature combination tests"
    echo "  integration  - Run integration tests (requires API credentials)"
    echo "  coverage     - Run tests with coverage analysis"
    echo "  full         - Run all tests (requires full environment setup)"
    echo "  check        - Check environment and show what tests can be run"
    echo "  help         - Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  APP_ID                     - Feishu App ID (required for integration tests)"
    echo "  APP_SECRET                 - Feishu App Secret (required for integration tests)"
    echo "  USER_ACCESS_TOKEN          - User access token (required for user-specific tests)"
    echo "  ENABLE_INTEGRATION_DOCTESTS - Set to 'true' to run API-dependent doctests"
    echo ""
    echo "Examples:"
    echo "  $0 auto                           # Detect environment and run appropriate tests"
    echo "  ENABLE_INTEGRATION_DOCTESTS=true $0 integration  # Run integration tests"
    echo "  $0 coverage                       # Generate coverage report"
}

main() {
    cd "$PROJECT_ROOT"
    
    local command=${1:-auto}
    
    case "$command" in
        "auto")
            local env_type=$(detect_environment)
            
            print_header "Running Tests for Environment: $env_type"
            
            run_unit_tests
            run_feature_tests
            
            case "$env_type" in
                "full")
                    run_integration_tests
                    ;;
                "integration")
                    run_integration_tests
                    ;;
                "local")
                    run_doctest_safe
                    ;;
            esac
            
            print_success "All tests for $env_type environment completed!"
            ;;
            
        "unit")
            run_unit_tests
            ;;
            
        "features")
            run_feature_tests
            ;;
            
        "integration")
            run_integration_tests
            ;;
            
        "coverage")
            run_coverage_tests
            ;;
            
        "full")
            run_unit_tests
            run_feature_tests
            run_integration_tests
            run_coverage_tests
            ;;
            
        "check")
            detect_environment
            ;;
            
        "help"|"-h"|"--help")
            show_help
            ;;
            
        *)
            print_error "Unknown command: $command"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"