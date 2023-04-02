#!/bin/bash

cmd=${1}

server_url="localhost:8090"

if [ $cmd = 'user-create' ]; then
  curl \
    -X POST \
     -H 'Content-type: application/json' \
    -d '{ "username": "user5" }' \
    $server_url/users

else
  echo "not recognized: ${cmd}"
fi
