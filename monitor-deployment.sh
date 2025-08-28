#!/bin/bash

# Monitor deployment and send Pushover notification
PUSHOVER_USER_KEY="udgzv2qobozqw2nkryw89vqh9g9b41"
PUSHOVER_APP_TOKEN="azp41vja5xsa9zgvnsxkw8qps8d6wq"  # Using a generic app token
URL="https://simbo1905.github.io/ross-sea-food-web/"
MAX_ATTEMPTS=20  # 20 attempts * 30 seconds = 10 minutes max
ATTEMPT=0

echo "Monitoring deployment at: $URL"
echo "Will check every 30 seconds for up to 10 minutes..."
echo ""

while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    ATTEMPT=$((ATTEMPT + 1))
    echo -n "Attempt $ATTEMPT/$MAX_ATTEMPTS: "
    
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$URL")
    
    if [ "$HTTP_STATUS" -eq 200 ]; then
        echo "✅ Site is LIVE! (HTTP 200)"
        
        # Test content
        CONTENT=$(curl -s "$URL")
        if echo "$CONTENT" | grep -q "Ross Sea Food Web Quiz" && \
           echo "$CONTENT" | grep -q "University of Waikato"; then
            echo "✅ Content verified!"
            
            # Send success notification
            curl -s \
                --form-string "token=$PUSHOVER_APP_TOKEN" \
                --form-string "user=$PUSHOVER_USER_KEY" \
                --form-string "title=Ross Sea Food Web Quiz - Deployment Successful! 🎉" \
                --form-string "message=The game is now live at: $URL" \
                --form-string "url=$URL" \
                --form-string "url_title=Play the game" \
                --form-string "priority=1" \
                https://api.pushover.net/1/messages.json > /dev/null
            
            echo ""
            echo "✅ Pushover notification sent!"
            exit 0
        else
            echo "⚠️  Site is accessible but content verification failed"
        fi
    else
        echo "HTTP $HTTP_STATUS - Not ready yet"
    fi
    
    if [ $ATTEMPT -lt $MAX_ATTEMPTS ]; then
        sleep 30
    fi
done

# If we get here, deployment failed after all attempts
echo ""
echo "❌ Deployment did not become available after $MAX_ATTEMPTS attempts"

# Send failure notification
curl -s \
    --form-string "token=$PUSHOVER_APP_TOKEN" \
    --form-string "user=$PUSHOVER_USER_KEY" \
    --form-string "title=Ross Sea Food Web Quiz - Deployment Failed ❌" \
    --form-string "message=The deployment did not become available after 10 minutes. URL: $URL" \
    --form-string "priority=0" \
    https://api.pushover.net/1/messages.json > /dev/null

echo "❌ Failure notification sent to Pushover"
exit 1