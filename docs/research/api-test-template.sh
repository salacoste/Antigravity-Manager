#!/bin/bash

################################################################################
# Epic-020 API Testing Template
# Purpose: Test Model IDs 314-327 across Vertex AI and AI Studio
# Date: 2026-01-13 (Day 1 Afternoon - Prepared for Day 2 Execution)
# Author: Dev C (Junior Developer)
#
# Usage:
#   ./api-test-template.sh --project-id YOUR_PROJECT_ID --api-key YOUR_API_KEY
#
# Requirements:
#   - curl (HTTP client)
#   - jq (JSON parser, optional but recommended)
#   - gcloud CLI (for Vertex AI authentication)
#   - Valid Google Cloud project with APIs enabled
################################################################################

set -e  # Exit on error

################################################################################
# Configuration
################################################################################

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
MODELS=(314 315 316 317 318 319 320 321 322 323 324 325 326 327)
PROJECT_ID="${1:-your-project-id}"
API_KEY="${2:-your-api-key}"
LOG_DIR="logs/day2-api-testing"
TEST_DATE=$(date +%Y-%m-%d)
RESULTS_FILE="${LOG_DIR}/model-tests-${TEST_DATE}.json"
TIMEOUT=30  # Seconds per request

# API endpoints
VERTEX_AI_BASE="https://us-central1-aiplatform.googleapis.com/v1"
AI_STUDIO_BASE="https://generativelanguage.googleapis.com/v1beta"

################################################################################
# Functions
################################################################################

# Initialize logging
init_logging() {
    mkdir -p "${LOG_DIR}"
    echo "📝 Test Harness Initialized"
    echo "   Date: ${TEST_DATE}"
    echo "   Project: ${PROJECT_ID}"
    echo "   Log Directory: ${LOG_DIR}"
    echo ""
}

# Print colored output
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Validate prerequisites
validate_prereqs() {
    log_info "Validating prerequisites..."

    # Check curl
    if ! command -v curl &> /dev/null; then
        log_error "curl not found. Please install curl."
        exit 1
    fi
    log_success "✓ curl found"

    # Check jq (optional)
    if command -v jq &> /dev/null; then
        log_success "✓ jq found (JSON parsing enabled)"
        JQ_AVAILABLE=true
    else
        log_warning "jq not found (JSON output will not be pretty-printed)"
        JQ_AVAILABLE=false
    fi

    # Check gcloud (optional)
    if command -v gcloud &> /dev/null; then
        log_success "✓ gcloud found"
    else
        log_warning "gcloud not found (manual authentication may be needed)"
    fi

    # Check network connectivity
    if ! ping -c 1 8.8.8.8 &> /dev/null; then
        log_error "No internet connectivity detected"
        exit 1
    fi
    log_success "✓ Network connectivity verified"

    echo ""
}

# Test 1: Vertex AI - List Models Endpoint
test_vertex_ai_list_models() {
    local model_id=$1
    local endpoint="${VERTEX_AI_BASE}/projects/${PROJECT_ID}/locations/us-central1/models"

    log_info "Testing Vertex AI list models for model ${model_id}..."

    local response=$(curl -s -w "\n%{http_code}" \
        -H "Authorization: Bearer $(gcloud auth application-default print-access-token)" \
        -H "Content-Type: application/json" \
        --max-time ${TIMEOUT} \
        "${endpoint}" 2>&1 || echo "")

    local http_code=$(echo "$response" | tail -n1)
    local body=$(echo "$response" | head -n-1)
    local response_time=$(date +%s%N)

    echo "${http_code}|${body}|${response_time}"
}

# Test 2: Vertex AI - Direct Model Endpoint
test_vertex_ai_direct_model() {
    local model_id=$1
    local endpoint="${VERTEX_AI_BASE}/projects/${PROJECT_ID}/locations/us-central1/publishers/google/models/${model_id}"

    log_info "Testing Vertex AI direct model endpoint for model ${model_id}..."

    local response=$(curl -s -w "\n%{http_code}" \
        -H "Authorization: Bearer $(gcloud auth application-default print-access-token)" \
        -H "Content-Type: application/json" \
        --max-time ${TIMEOUT} \
        "${endpoint}" 2>&1 || echo "")

    local http_code=$(echo "$response" | tail -n1)
    local body=$(echo "$response" | head -n-1)

    echo "${http_code}|${body}"
}

# Test 3: AI Studio - Chat Completion
test_ai_studio_chat_completion() {
    local model_id=$1
    local endpoint="${AI_STUDIO_BASE}/models/${model_id}:generateContent?key=${API_KEY}"

    log_info "Testing AI Studio chat completion for model ${model_id}..."

    local request_body='{
        "contents": {
            "parts": [
                {
                    "text": "Hello, can you respond?"
                }
            ]
        }
    }'

    local response=$(curl -s -w "\n%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        --max-time ${TIMEOUT} \
        -d "${request_body}" \
        "${endpoint}" 2>&1 || echo "")

    local http_code=$(echo "$response" | tail -n1)
    local body=$(echo "$response" | head -n-1)

    echo "${http_code}|${body}"
}

# Test 4: AI Studio - List Models
test_ai_studio_list_models() {
    local endpoint="${AI_STUDIO_BASE}/models?key=${API_KEY}"

    log_info "Testing AI Studio list models..."

    local response=$(curl -s -w "\n%{http_code}" \
        -H "Content-Type: application/json" \
        --max-time ${TIMEOUT} \
        "${endpoint}" 2>&1 || echo "")

    local http_code=$(echo "$response" | tail -n1)
    local body=$(echo "$response" | head -n-1)

    echo "${http_code}|${body}"
}

# Test single model against all endpoints
test_model() {
    local model_id=$1

    echo ""
    log_info "====== Testing Model ID: ${model_id} ======"

    # Test Vertex AI List
    log_info "Test 1/3: Vertex AI List Models"
    local va_list=$(test_vertex_ai_list_models ${model_id})
    local va_list_code=$(echo "${va_list}" | cut -d'|' -f1)
    if [ "${va_list_code}" = "200" ]; then
        log_success "Response: ${va_list_code}"
    else
        log_warning "Response: ${va_list_code}"
    fi

    # Test Vertex AI Direct
    log_info "Test 2/3: Vertex AI Direct Model"
    local va_direct=$(test_vertex_ai_direct_model ${model_id})
    local va_direct_code=$(echo "${va_direct}" | cut -d'|' -f1)
    if [ "${va_direct_code}" = "404" ]; then
        log_success "Response: ${va_direct_code} (Expected for missing models)"
    elif [ "${va_direct_code}" = "200" ]; then
        log_success "Response: ${va_direct_code} (Model found!)"
    else
        log_warning "Response: ${va_direct_code}"
    fi

    # Test AI Studio Chat
    log_info "Test 3/3: AI Studio Chat Completion"
    local as_chat=$(test_ai_studio_chat_completion ${model_id})
    local as_chat_code=$(echo "${as_chat}" | cut -d'|' -f1)
    if [ "${as_chat_code}" = "404" ]; then
        log_success "Response: ${as_chat_code} (Expected for missing models)"
    elif [ "${as_chat_code}" = "200" ]; then
        log_success "Response: ${as_chat_code} (Model working!)"
    else
        log_warning "Response: ${as_chat_code}"
    fi

    # Record results
    local result_json=$(cat <<EOF
    {
        "model_id": ${model_id},
        "tests": {
            "vertex_ai_list": ${va_list_code},
            "vertex_ai_direct": ${va_direct_code},
            "ai_studio_chat": ${as_chat_code}
        },
        "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    }
EOF
)

    echo "${result_json}" >> "${RESULTS_FILE}.tmp"
}

# Run all model tests
run_all_tests() {
    echo ""
    log_info "Starting API tests for models: ${MODELS[@]}"
    echo ""

    # Initialize results file
    echo "{" > "${RESULTS_FILE}.tmp"
    echo "  \"test_date\": \"${TEST_DATE}\"," >> "${RESULTS_FILE}.tmp"
    echo "  \"total_models\": ${#MODELS[@]}," >> "${RESULTS_FILE}.tmp"
    echo "  \"models\": [" >> "${RESULTS_FILE}.tmp"

    local total=${#MODELS[@]}
    local count=0

    for model_id in "${MODELS[@]}"; do
        count=$((count + 1))
        test_model ${model_id}

        # Progress indicator
        echo "Progress: ${count}/${total}"
    done

    # Close JSON
    echo "  ]" >> "${RESULTS_FILE}.tmp"
    echo "}" >> "${RESULTS_FILE}.tmp"

    # Format JSON if jq available
    if [ "${JQ_AVAILABLE}" = true ]; then
        jq '.' "${RESULTS_FILE}.tmp" > "${RESULTS_FILE}"
        rm "${RESULTS_FILE}.tmp"
    else
        mv "${RESULTS_FILE}.tmp" "${RESULTS_FILE}"
    fi
}

# Analyze results
analyze_results() {
    echo ""
    log_info "====== Analysis Summary ======"
    echo ""

    if command -v jq &> /dev/null; then
        # Count different response codes
        local total_tests=$(jq '.models | length' "${RESULTS_FILE}")
        local not_found_count=$(jq '[.models[] | select(.tests.vertex_ai_direct == 404)] | length' "${RESULTS_FILE}")
        local found_count=$(jq '[.models[] | select(.tests.vertex_ai_direct == 200)] | length' "${RESULTS_FILE}")

        echo "Total models tested: ${total_tests}"
        echo "404 Not Found: ${not_found_count}"
        echo "200 OK (Found): ${found_count}"
        echo ""

        if [ "${not_found_count}" -eq "${total_tests}" ]; then
            log_success "✓ All models returned 404 - Confirms DEPRECATED hypothesis"
        elif [ "${found_count}" -gt 0 ]; then
            log_success "✓ Found ${found_count} active models - Confirms EXTERNAL hypothesis"
        else
            log_warning "Mixed results - Possible RESERVED models or API issues"
        fi
    else
        log_info "Results saved to: ${RESULTS_FILE}"
        log_info "Use jq to analyze: jq '.' ${RESULTS_FILE}"
    fi

    echo ""
}

# Display usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

OPTIONS:
    --project-id ID     Google Cloud project ID (required)
    --api-key KEY       Google AI Studio API key (required)
    --help              Display this help message

EXAMPLE:
    $0 --project-id my-project-id --api-key my-api-key

ENVIRONMENT VARIABLES:
    GCP_PROJECT_ID     Alternatively, set project ID via env var
    GOOGLE_API_KEY     Alternatively, set API key via env var

REQUIREMENTS:
    - curl              (for HTTP requests)
    - jq                (optional, for JSON formatting)
    - gcloud CLI        (for Vertex AI authentication)

OUTPUT:
    Results saved to: logs/day2-api-testing/model-tests-YYYY-MM-DD.json

EOF
    exit 0
}

################################################################################
# Main Script
################################################################################

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --project-id)
            PROJECT_ID="$2"
            shift 2
            ;;
        --api-key)
            API_KEY="$2"
            shift 2
            ;;
        --help)
            usage
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            ;;
    esac
done

# Validate inputs
if [ "${PROJECT_ID}" = "your-project-id" ] || [ -z "${PROJECT_ID}" ]; then
    log_error "Project ID is required"
    usage
fi

if [ "${API_KEY}" = "your-api-key" ] || [ -z "${API_KEY}" ]; then
    log_error "API key is required"
    usage
fi

# Run tests
echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║  Epic-020 API Testing - Models 314-327                 ║"
echo "║  Date: ${TEST_DATE}                                    ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

init_logging
validate_prereqs
run_all_tests
analyze_results

echo "✅ Testing complete!"
echo "Results saved to: ${RESULTS_FILE}"
echo ""
log_info "Next steps:"
echo "  1. Review results: cat ${RESULTS_FILE}"
echo "  2. Update tracking matrix with findings"
echo "  3. Run Day 2 findings analysis"
echo ""
