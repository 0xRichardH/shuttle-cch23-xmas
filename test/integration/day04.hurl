###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###



# ⭐ Task 1: Reindeer cheer
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

# 🎁 Task 2: Cursed candy eating contest (150 bonus points)
POST {{host}}/4/contest
Content-Type: application/json
```json
[
    {
      "name": "Dasher",
      "strength": 5,
      "speed": 50.4,
      "height": 80,
      "antler_width": 36,
      "snow_magic_power": 9001,
      "favorite_food": "hay",
      "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
      "name": "Dancer",
      "strength": 6,
      "speed": 48.2,
      "height": 65,
      "antler_width": 37,
      "snow_magic_power": 4004,
      "favorite_food": "grass",
      "cAnD13s_3ATeN-yesT3rdAy": 5
    }
  ]
```
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.fastest" == "Speeding past the finish line with a strength of 5 is Dasher"
jsonpath "$.tallest" == "Dasher is standing tall with his 36 cm wide antlers"
jsonpath "$.magician" == "Dasher could blast you away with a snow magic power of 9001"
jsonpath "$.consumer" == "Dancer ate lots of candies, but also some grass"
