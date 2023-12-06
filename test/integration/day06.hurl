###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###

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
