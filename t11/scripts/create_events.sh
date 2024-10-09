#!/usr/bin/env bash

address=http://127.0.0.1:3000

echo "API tests using $address as service address."

echo "Testing $address/create_event..."
curl -X POST $address/create_event \
-H "Content-Type: application/json" \
-d '{"user_id": 1, "date": "2024-01-01", "description": "Reading"}' \
-i && echo "\n"

curl -X POST $address/create_event \
-H "Content-Type: application/json" \
-d '{"user_id": 1, "date": "2024-01-05", "description": "Running"}' \
-i && echo "\n"

curl -X POST $address/create_event \
-H "Content-Type: application/json" \
-d '{"user_id": 1, "date": "2024-01-25", "description": "Cooking"}' \
-i && echo "\n"
