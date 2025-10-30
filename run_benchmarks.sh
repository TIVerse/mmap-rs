#!/bin/bash
# Benchmark execution script for mmap-rs

echo "======================================"
echo "⚡ Running mmap-rs Benchmarks"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}📊 Running Sequential Access Benchmarks${NC}"
echo "This will take several minutes..."
echo ""

# Run sequential benchmarks
cargo bench --bench sequential 2>&1 | tee benchmark_sequential.log

echo ""
echo -e "${BLUE}📊 Running Random Access Benchmarks${NC}"
echo "This will take several minutes..."
echo ""

# Run random access benchmarks
cargo bench --bench random 2>&1 | tee benchmark_random.log

echo ""
echo "======================================"
echo -e "${GREEN}✅ Benchmarks Complete${NC}"
echo "======================================"
echo ""
echo "Results saved to:"
echo "  - benchmark_sequential.log"
echo "  - benchmark_random.log"
echo "  - target/criterion/ (HTML reports)"
echo ""
echo "To view HTML reports:"
echo "  firefox target/criterion/report/index.html"
echo "  # or"
echo "  xdg-open target/criterion/report/index.html"
echo ""
