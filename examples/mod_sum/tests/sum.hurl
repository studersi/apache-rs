GET http://localhost:8082/index.html.sum
HTTP 200

[Asserts]
body contains "Information on /var/www/html/index.html:"
body contains "<b>Size:</b> 10671 bytes"
body contains "<b>digest type:</b> -"
body contains "<b>digest:</b> <code>no hashing algorithm specified</code>"
