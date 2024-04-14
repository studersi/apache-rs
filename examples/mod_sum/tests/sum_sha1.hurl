GET http://localhost:8082/index.html.sum?digest=sha1
HTTP 200

[Asserts]
body contains "Information on /var/www/html/index.html:"
body contains "<b>Size:</b> 10671 bytes"
body contains "<b>digest type:</b> sha1"
body contains "<b>digest:</b> <code>152a75d80c0bdadb382e1cafe517159cb76a19cc</code>"
