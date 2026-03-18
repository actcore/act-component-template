POST {% raw %}{{baseurl}}{% endraw %}/tools
HTTP 200
[Asserts]
jsonpath "$.tools" count >= 1
