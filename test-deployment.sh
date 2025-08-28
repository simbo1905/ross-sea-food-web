#!/bin/bash

# Test deployment script for Ross Sea Food Web Quiz
URL="https://simbo1905.github.io/ross-sea-food-web/"

echo "Testing deployment at: $URL"
echo "================================"

# Test if page is accessible
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$URL")
echo "HTTP Status: $HTTP_STATUS"

if [ "$HTTP_STATUS" -eq 200 ]; then
    echo "✅ Page is accessible"
    
    # Download page content
    CONTENT=$(curl -s "$URL")
    
    # Check for key elements
    echo ""
    echo "Checking page content..."
    
    if echo "$CONTENT" | grep -q "Ross Sea Food Web Quiz"; then
        echo "✅ Page title found"
    else
        echo "❌ Page title not found"
    fi
    
    if echo "$CONTENT" | grep -q "University of Waikato"; then
        echo "✅ Copyright attribution found"
    else
        echo "❌ Copyright attribution not found"
    fi
    
    if echo "$CONTENT" | grep -q "sciencelearn.org.nz"; then
        echo "✅ Science Learning Hub link found"
    else
        echo "❌ Science Learning Hub link not found"
    fi
    
    if echo "$CONTENT" | grep -q "M. Pinkerton"; then
        echo "✅ M. Pinkerton attribution found"
    else
        echo "❌ M. Pinkerton attribution not found"
    fi
    
    if echo "$CONTENT" | grep -q "EMBEDDED_QUESTION_SETS"; then
        echo "✅ Question data embedded"
    else
        echo "❌ Question data not found"
    fi
    
else
    echo "❌ Page is not accessible (HTTP $HTTP_STATUS)"
    echo "The site may not be deployed yet."
    echo "After pushing to GitHub, wait a few minutes and try again."
fi

echo ""
echo "================================"
echo "Note: GitHub Pages deployment can take 5-10 minutes after the first push."