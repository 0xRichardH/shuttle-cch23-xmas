###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


POST {{host}}/4/strength
Content-Type: application/json
```json
[
  { "name": "Dasher", "strength": 5 },
  { "name": "Dancer", "strength": 6 },
  { "name": "Prancer", "strength": 4 },
  { "name": "Vixen", "strength": 7 }
]
```
HTTP 200
`22`
