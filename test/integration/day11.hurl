###
# hurl --variable host=http://localhost:8000 --verbose test.hurl
###


# ⭐ Task 1: Served on a silver platter
GET {{host}}/11/assets/decoration.png
HTTP 200
[Asserts]
header "Content-Type" == "image/png"
header "Content-Length" == "787297"

# 🎁 Task 2: Bull mode activated (200 bonus points)
# curl -X POST http://localhost:8000/11/red_pixels \
#   -H 'Content-Type: multipart/form-data' \
#   -F 'image=@decoration.png' # the image from Task 1
