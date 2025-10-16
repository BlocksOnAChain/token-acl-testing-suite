#!/bin/bash

# Token ACL Testing Suite - Clean Script
# Cleans all build artifacts and test reports

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
ALL=false
REPORTS=false
BUILD=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -a|--all)
            ALL=true
            shift
            ;;
        -r|--reports)
            REPORTS=true
            shift
            ;;
        -b|--build)
            BUILD=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -a, --all        Clean everything (build artifacts and reports)"
            echo "  -b, --build      Clean build artifacts only"
            echo "  -r, --reports    Clean test reports only"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                # Clean build artifacts (default)"
            echo "  $0 --all          # Clean everything"
            echo "  $0 --reports      # Clean test reports only"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# If no specific option is provided, default to cleaning build artifacts
if [ "$ALL" = false ] && [ "$REPORTS" = false ] && [ "$BUILD" = false ]; then
    BUILD=true
fi

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   sRFC 37: Token ACL Testing Suite - Clean                       ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Function to print status
print_status() {
    echo -e "${BLUE}►${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Clean build artifacts
if [ "$ALL" = true ] || [ "$BUILD" = true ]; then
    print_status "Cleaning build artifacts..."
    
    # Clean integration tests
    if [ -d "tests/integration/target" ]; then
        cd tests/integration
        cargo clean
        cd ../..
        print_success "Integration tests build artifacts cleaned"
    fi
    
    # Clean example allow list
    if [ -d "examples/allow_list/target" ]; then
        cd examples/allow_list
        cargo clean
        cd ../..
        print_success "Example allow list build artifacts cleaned"
    fi
    
    # Clean example block list
    if [ -d "examples/block_list/target" ]; then
        cd examples/block_list
        cargo clean
        cd ../..
        print_success "Example block list build artifacts cleaned"
    fi
    
    # Clean production allow list
    if [ -d "programs/production_allow_list/target" ]; then
        cd programs/production_allow_list
        cargo clean
        cd ../..
        print_success "Production allow list build artifacts cleaned"
    fi
    
    # Clean workspace target directory
    if [ -d "target" ]; then
        rm -rf target
        print_success "Workspace build artifacts cleaned"
    fi
fi

# Clean test reports
if [ "$ALL" = true ] || [ "$REPORTS" = true ]; then
    print_status "Cleaning test reports..."
    
    if [ -d "tests/reports" ]; then
        # Remove generated report files but keep the directory
        rm -f tests/reports/*.md
        rm -f tests/reports/*.txt
        print_success "Test reports cleaned"
    fi
fi

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║                    CLEAN COMPLETE! ✓                             ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

if [ "$ALL" = true ]; then
    print_success "All build artifacts and test reports cleaned"
elif [ "$BUILD" = true ]; then
    print_success "Build artifacts cleaned"
elif [ "$REPORTS" = true ]; then
    print_success "Test reports cleaned"
fi

echo ""
echo "═══ NEXT STEPS ═══"
echo ""
echo "Setup environment:"
echo "  ${GREEN}./scripts/setup.sh${NC}"
echo ""
echo "Build programs:"
echo "  ${GREEN}./scripts/build.sh${NC}"
echo ""
echo "Run tests:"
echo "  ${GREEN}./scripts/test.sh${NC}"
echo ""
