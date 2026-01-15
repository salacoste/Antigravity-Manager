#!/bin/bash
# Quick test - first 5 models only with longer delays

API_KEY="sk-d0f78503acb64c6fb3b56a3569d43935"
ANTIGRAVITY_URL="http://127.0.0.1:8045"
PROMPT="PING"

echo "🧪 Quick Test - First 5 models"
echo "=============================="
echo ""

# Get account limits
ACCOUNTS=$(curl -s -H "x-api-key: $API_KEY" "$ANTIGRAVITY_URL/account-limits")
TOTAL_MODELS=$(echo "$ACCOUNTS" | python3 -c "
import json, sys
d = json.load(sys.stdin)
count = 0
for acc in d.get('accounts', []):
    limits = acc.get('limits', {})
    count += len(limits)
    if count >= 5:
        break
print(count)
")

echo "📊 Testing first 5 models (of $TOTAL_MODELS total)..."
echo ""

# Test first 5 models sequentially
TESTED=0
SUCCESS=0
FAILED=0

python3 <<PYTHON_SCRIPT | while IFS='|' read -r email model quota; do
    TESTED=$((TESTED + 1))
    
    echo -e "\033[0;34m🧪 [$TESTED/5] Testing:\033[0m $email"
    echo -e "   Model: \033[1;33m$model\033[0m (quota: \033[0;32m${quota}%\033[0m)"
    
    # Test with longer delay
    response=$(curl -s -X POST "$ANTIGRAVITY_URL/v1/messages" \
        -H "Content-Type: application/json" \
        -H "x-api-key: $API_KEY" \
        -d "{
            \"model\": \"$model\",
            \"max_tokens\": 50,
            \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT\"}]
        }" 2>&1)
    
    # Check result
    if echo "$response" | grep -q '"content"'; then
        SUCCESS=$((SUCCESS + 1))
        content=$(echo "$response" | python3 -c "import json, sys; d=json.load(sys.stdin); print(d.get('content', [{}])[0].get('text', 'OK')[:50])" 2>/dev/null)
        echo -e "   \033[0;32m✅ SUCCESS\033[0m: $content"
    elif echo "$response" | grep -q '"error"'; then
        FAILED=$((FAILED + 1))
        error=$(echo "$response" | python3 -c "import json, sys; d=json.load(sys.stdin); print(d.get('error', {}).get('message', 'Error')[:50])" 2>/dev/null)
        echo -e "   \033[0;31m❌ ERROR\033[0m: $error"
    else
        FAILED=$((FAILED + 1))
        echo -e "   \033[0;31m❌ INVALID JSON\033[0m"
        echo "   Raw: ${response:0:100}..."
    fi
    
    # Longer delay between tests
    sleep 2
    
    if [ $TESTED -ge 5 ]; then
        break
    fi
done < <(python3 -c "
import json, sys
d = json.load(sys.stdin)
count = 0
for acc in d.get('accounts', []):
    limits = acc.get('limits', {})
    for model, info in limits.items():
        if count >= 5:
            break
        print(f'{acc[\"email\"]}|{model}|{int(info.get(\"remaining_fraction\", 0) * 100)}')
        count += 1
    if count >= 5:
        break
" <<< "$ACCOUNTS")

echo ""
echo "=============================="
echo "📊 Results:"
echo "  Tested: $TESTED"
echo -e "  \033[0;32m✅ Success: $SUCCESS\033[0m"
echo -e "  \033[0;31m❌ Failed: $FAILED\033[0m"
echo ""

if [ $SUCCESS -gt 0 ]; then
    echo -e "\033[0;32m🎉 Some tests passed!\033[0m"
elif [ $FAILED -eq $TESTED ]; then
    echo -e "\033[0;33m⚠️  All rate limited (429) - quotas may be exhausted\033[0m"
fi
