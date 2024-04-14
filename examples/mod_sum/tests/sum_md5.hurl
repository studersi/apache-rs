GET http://localhost:8082/index.html.sum?digest=md5
HTTP 200

[Asserts]
body contains "Information on /var/www/html/index.html:"
body contains "<b>Size:</b> 10671 bytes"
body contains "<b>digest type:</b> md5"
body contains "<b>digest:</b> <code>720999b43a3be0674180354ac41f20b1</code>"
