### Task

Implement a simple telnet client in Rust.

Call examples:

    telnet --timeout=10s host port

    telnet mysite.ru 8080

    telnet --timeout=3s 1.1.1.1 123

#### Requirements

1. The program must connect to the specified host (ip or domain name + port) via TCP. After connecting, the STDIN of the program must be written to the socket, and the data received from the socket must be output to STDOUT

2. Optionally, you can pass a timeout to the program for connecting to the server (via the --timeout argument, 10s by default)

3. When you press Ctrl+D, the program must close the socket and exit. If the socket is closed from the server side, the program must also exit. When connecting to a non-existent server, the program must exit via timeout
