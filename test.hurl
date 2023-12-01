###
# hurl --verbose test.hurl
###

### Day -1: Get your winter boots on!

# Task 1: Everything is OK
GET http://localhost:8000/
Content-Type: application/json
HTTP 200

# Task 2: Fake error (0 bonus points)
GET http://localhost:8000/-1/error
HTTP 500
