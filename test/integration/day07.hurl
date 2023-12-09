###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


# ⭐ Task 1: Based encoding, 64th edition
GET {{host}}/7/decode
Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
HTTP 200
`{"flour":100,"chocolate chips":20}`


# 🎁 Task 2: The secret cookie recipe (120 bonus points)
GET {{host}}/7/bake
Cookie: recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319
HTTP 200
[Asserts]
jsonpath "$.cookies" == 4
jsonpath "$.pantry.flour" == 5
jsonpath "$.pantry.sugar" == 307
jsonpath "$.pantry.butter" == 2002
jsonpath "$.pantry['baking powder']" == 825
jsonpath "$.pantry.['chocolate chips']" == 257
