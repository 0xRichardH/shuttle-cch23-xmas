###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


### Day -1: Get your winter boots on!

# Task 1: Everything is OK
GET {{host}}/
HTTP 200
`Hello, world!`

# Task 2: Fake error (0 bonus points)
GET {{host}}/-1/error
HTTP 500
