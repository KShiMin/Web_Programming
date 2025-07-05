curl command:

//Create a new bug
e.g. 
curl -i -X POST http://127.0.0.1:8080/bugs/new \
  -H "Content-Type: application/json" \
  -d '{
        "title":       "Cannot login",
        "description": "Getting 500 when I submit the form",
        "reported_by": "alice",
        "severity":    "high"
      }'

//list bugs
curl -i http://127.0.0.1:8080/bugs

//list the high severity bug
curl -i "http://127.0.0.1:8080/bugs?status=open&severity=high"

//search/list the bugs with id
curl -i http://127.0.0.1:8080/bugs/1

//PATCH
e.g. for bug id 1
curl -i -X PATCH http://127.0.0.1:8080/bugs/1 \
  -H "Content-Type: application/json" \
  -d '{
        "status":       "in_progress",
        "developer_id": 2,
        "severity":     "medium"
      }'

//delete bug with id
curl -i -X DELETE http://127.0.0.1:8080/bugs/1

//error handling? with non-numeric id (e.g. foobar)
curl -i http://127.0.0.1:8080/bugs/foobar

//error handling with imcomplete details
curl -i -X POST http://127.0.0.1:8080/bugs/new \
  -H "Content-Type: application/json" \
  -d '{}'