###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


# ⭐ Task 1: IT'S PIKACHU!
GET {{host}}/8/weight/25
HTTP 200
`6`

# 🎁 Task 2: That's gonna leave a dent (160 bonus points)
GET {{host}}/8/drop/25
HTTP 200
`84.10707461325713`
