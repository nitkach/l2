#!/usr/bin/env bash

address=http://127.0.0.1:3000

echo "API tests using $address as service address."

echo "Testing $address/events_for_day?user_id=1&date=2024-01-01..."
curl -X GET $address'/events_for_day?user_id=1&date=2024-01-01' \
-i && echo "\n"

echo "Testing $address/events_for_day?user_id=1&date=2024-02-01..."
curl -X GET $address'/events_for_day?user_id=1&date=2024-02-01' \
-i && echo "\n"

echo "Testing $address/events_for_day?user_id=1&date=2024-01-25..."
curl -X GET $address'/events_for_day?user_id=1&date=2024-01-25' \
-i && echo "\n"
