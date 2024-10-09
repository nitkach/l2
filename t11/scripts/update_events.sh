#!/usr/bin/env bash

address=http://127.0.0.1:3000
ulid=NOT_SPECIFIED

echo "API tests using $address as service address."
echo "To test this endpoint please run create_events.sh and events_for_day.sh to get the ulid of the event you want to update"
echo "Current ulid: $ulid\n"

echo "Testing $address/update_event..."
curl -X POST $address/update_event \
-H "Content-Type: application/json" \
-d '{"user_id": 1, "date": "2024-01-01", "description": "Bookiehorsing", "ulid": "'$ulid'"}' \
-i && echo "\n"
