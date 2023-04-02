#!/bin/bash

cmd=${1}

if [ $cmd = 'dev' ]; then
  cargo watch -x 'run'

else
  echo "not recognized: ${cmd}"
fi
