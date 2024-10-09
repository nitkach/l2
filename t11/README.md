### Task

Implement an HTTP server for working with a calendar. The task requires using the axum web framework.


#### Requirements

1. Implement helper functions for serializing domain objects to JSON.

2. Implement helper functions for parsing and validating parameters of the /create_event and /update_event methods.

3. Implement HTTP handlers for each API method using helper functions and domain objects.

4. Implement middleware for logging requests

5. Implement all methods.

6. Business logic should not depend on the HTTP server code.

7. In case of a business logic error, the server should return HTTP 503. In case of an input data error (invalid int, for example), the server should return HTTP 400. In case of other errors, the server should return HTTP 500. The web server should run on the port specified in the config and output each processed request to the log.


#### Project structure

API methods:

    POST /create_event

    POST /update_event

    POST /delete_event

    GET /events_for_day

    GET /events_for_week

    GET /events_for_month

Parameters are passed as `www-url-form-encoded` (i.e. regular `user_id=3&date=2019-09-09`). In GET methods, parameters are passed via query string, in POST via the request body.

Each request should return a JSON document containing either `{"result": "..."}` in case of successful method execution, or `{"error": "..."}` in case of a business logic error.
