###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


# ⭐ Task 1: Never count on an elf
POST {{host}}/6
Content-Type: text/plain
```
The mischievous elf peeked out from behind the toy workshop,
      and another elf joined in the festive dance.
      Look, there is also an elf on that shelf!
```
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.elf" == 4

# Task 2: Shelf under an elf? (200 bonus points)
POST {{host}}/6
Content-Type: text/plain
```
there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.
```
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.elf" == 5
jsonpath "$['elf on a shelf']" == 1
jsonpath "$['shelf with no elf on it']" == 1

