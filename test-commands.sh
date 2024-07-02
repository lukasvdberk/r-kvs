#!/bin/bash

# test curl commands for the api

host="http://localhost:8080"
# add an key
curl -X POST -H "Content-Type: application/json" -d '{"key": "key1", "value": "value1"}' $host/table1/key1

# read a key
curl -X GET $host/table1/key1