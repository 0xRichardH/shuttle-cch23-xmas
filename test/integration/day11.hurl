###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###

GET {{host}}/11/assets/decoration.png
HTTP 200
[Asserts]
header "Content-Type" == "image/png"
header "Content-Length" == "787297"
