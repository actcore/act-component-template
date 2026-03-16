GET {{baseurl}}/info
HTTP 200
[Asserts]
jsonpath "$['std:name']" == "{{project-name}}"
jsonpath "$['std:version']" isString
