#!/bin/bash

cmd=${1:-up}
project_name=""
file_prefix=""
container_name="workspace"

execute-docker-compose () {
  opts="-f ${file_prefix}docker-compose.yml"

  if [ -n "$project_name" ]; then
    opts="$opts -p $project_name"
  fi

  docker-compose $opts $@
}

stop-docker-compose () {
  execute-docker-compose stop
}

if [ $cmd = 'up' ] && [ $# -le 1 ]; then
  execute-docker-compose up -d
  execute-docker-compose exec $container_name /bin/bash
  stop-docker-compose

elif [ $cmd = 'stop' ]; then
  stop-docker-compose

elif [ $cmd = 'bash' ]; then
  execute-docker-compose exec $container_name /bin/bash

# sea-orm-cli generate entity -o ./entity/src

elif [ $cmd = 'bash-db' ]; then
  execute-docker-compose exec rstwitter-db /bin/bash

elif [ $cmd = 'mysql' ]; then
  execute-docker-compose exec rstwitter-db mysql -u root -ppassword rstwitter


else
  execute-docker-compose $@
fi
