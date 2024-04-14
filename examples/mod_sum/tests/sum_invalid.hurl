GET http://localhost:8082/index.html.sum?digest=invalid
HTTP 200

[Asserts]
body contains "Information on /var/www/html/index.html:"
body contains "<b>Size:</b> 10671 bytes"
body contains "<b>digest type:</b> invalid"
body contains "<b>digest:</b> <code>unknown type, could not calculate has sum</code>"
