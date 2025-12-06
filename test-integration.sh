#!/bin/bash

# VDownloader Integration Test Script
# Tests the complete frontend-backend integration

set -e

BASE_URL="http://localhost:4000"
API_URL="$BASE_URL/api"

echo "🧪 VDownloader Integration Test Suite"
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
pass() {
    echo -e "${GREEN}✓${NC} $1"
}

fail() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

# Check if server is running
check_server() {
    info "Checking if server is running..."
    if curl -s "$API_URL/health" > /dev/null 2>&1; then
        pass "Server is running"
    else
        fail "Server is not running. Start it with: npm start"
    fi
}

# Test health endpoint
test_health() {
    info "Testing health endpoint..."
    response=$(curl -s "$API_URL/health")
    
    if echo "$response" | grep -q "ok"; then
        pass "Health endpoint working"
    else
        fail "Health endpoint failed"
    fi
}

# Test platforms endpoint
test_platforms() {
    info "Testing platforms endpoint..."
    response=$(curl -s "$API_URL/platforms")
    
    if echo "$response" | grep -q "youtube"; then
        pass "Platforms endpoint working"
    else
        fail "Platforms endpoint failed"
    fi
}

# Test URL validation
test_validation() {
    info "Testing URL validation..."
    response=$(curl -s -X POST "$API_URL/validate" \
        -H "Content-Type: application/json" \
        -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}')
    
    if echo "$response" | grep -q '"success":true'; then
        pass "URL validation working"
    else
        fail "URL validation failed"
    fi
}

# Test download initiation
test_download() {
    info "Testing download initiation..."
    response=$(curl -s -X POST "$API_URL/download" \
        -H "Content-Type: application/json" \
        -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","format":"video","quality":"720p"}')
    
    if echo "$response" | grep -q '"downloadId"'; then
        download_id=$(echo "$response" | grep -o '"downloadId":"[^"]*"' | cut -d'"' -f4)
        pass "Download initiation working (ID: $download_id)"
        
        # Wait for download to process
        sleep 2
        
        # Test status endpoint
        info "Testing download status..."
        status_response=$(curl -s "$API_URL/status/$download_id")
        
        if echo "$status_response" | grep -q '"downloadId"'; then
            pass "Download status endpoint working"
        else
            fail "Download status endpoint failed"
        fi
    else
        fail "Download initiation failed"
    fi
}

# Test frontend files
test_frontend() {
    info "Testing frontend files..."
    
    if curl -s "$BASE_URL" | grep -q "VDownloader"; then
        pass "Frontend HTML accessible"
    else
        fail "Frontend HTML not accessible"
    fi
    
    if curl -s "$BASE_URL/styles.css" | grep -q "body"; then
        pass "Frontend CSS accessible"
    else
        fail "Frontend CSS not accessible"
    fi
    
    if curl -s "$BASE_URL/script.js" | grep -q "VDownloader"; then
        pass "Frontend JavaScript accessible"
    else
        fail "Frontend JavaScript not accessible"
    fi
}

# Run all tests
echo "Starting tests..."
echo ""

check_server
echo ""

test_health
test_platforms
test_validation
test_download
echo ""

test_frontend
echo ""

echo "======================================"
echo -e "${GREEN}All tests passed!${NC} ✨"
echo ""
echo "You can now:"
echo "  1. Open $BASE_URL in your browser"
echo "  2. Try downloading a video"
echo "  3. Watch real-time progress with WebSockets"
echo ""
