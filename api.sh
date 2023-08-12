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


elif [ $cmd = 'tweet-list' ]; then
  curl \
    -X GET \
    -H 'Content-type: application/json' \
    $server_url/tweets | jq


elif [ $cmd = 'tweet-create' ]; then
  curl \
    -X POST \
    -H 'Content-type: application/json' \
    -d '{ "user_id": 1, "content": "hoge kore ha iine" }' \
    $server_url/tweets | jq

else
  echo "not recognized: ${cmd}"
fi
