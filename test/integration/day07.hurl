###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


# ⭐ Task 1: Based encoding, 64th edition
GET {{host}}/7/decode
Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
HTTP 200
`{"flour":100,"chocolate chips":20}`
