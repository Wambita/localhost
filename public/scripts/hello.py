#!/usr/bin/env python3
import os
import sys

# CGI scripts should print headers first
print("Content-Type: text/html\r\n\r\n")

# Then print the body
print("<html>")
print("<head><title>CGI Test</title></head>")
print("<body>")
print("<h1>Hello from CGI!</h1>")
print("<p>Environment Variables:</p>")
print("<ul>")
for key, value in sorted(os.environ.items()):
    print(f"<li><strong>{key}</strong>: {value}</li>")
print("</ul>")

# Print any input received
if os.environ.get("REQUEST_METHOD") == "POST":
    content_length = int(os.environ.get("CONTENT_LENGTH", 0))
    body = sys.stdin.read(content_length)
    print("<h2>Received POST data:</h2>")
    print(f"<pre>{body}</pre>")

print("</body>")
print("</html>")