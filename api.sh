#!/bin/bash

cmd=${1}

server_url="localhost:8090"

if [ $cmd = 'user-list' ]; then
  curl \
    -X GET \
    -H 'Content-type: application/json' \
    $server_url/users | jq


elif [ $cmd = 'user-create' ]; then
  curl \
    -X POST \
    -H 'Content-type: application/json' \
    -d '{ "username": "user3" }' \
    $server_url/users | jq

else
  echo "not recognized: ${cmd}"
fi
