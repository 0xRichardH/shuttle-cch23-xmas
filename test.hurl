###
# hurl --verbose test.hurl
###


### Day -1: Get your winter boots on!

# Task 1: Everything is OK
GET http://localhost:8000/
HTTP 200
`Hello, world!`

# Task 2: Fake error (0 bonus points)
GET http://localhost:8000/-1/error
HTTP 500


### Day 1: Packet "exclusive-cube" not found


# ⭐ Task 1: Cube the bits
GET http://localhost:8000/1/4/8
HTTP 200
`1728`
