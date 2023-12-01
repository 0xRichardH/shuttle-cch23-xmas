###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


### Day 1: Packet "exclusive-cube" not found


# ⭐ Task 1: Cube the bits
GET {{host}}/1/4/8
HTTP 200
`1728`

# 🎁 Task 2: The sled ID system (100 bonus points)
GET {{host}}/1/10
HTTP 200
`1000`

GET {{host}}/1/4/5/8/10
HTTP 200
`27`

GET {{host}}/1/2/3/4/5/6/7/8/9/1/2/3/4/5/6/7/8/9/12/11/13/14
HTTP 404
