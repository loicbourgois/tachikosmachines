#!/bin/sh
# $HOME/github.com/loicbourgois/tachikosmachines/front/_go.sh
full_path=$HOME/github.com/loicbourgois/tachikosmachines/front/
echo "Frontend at http://0.0.0.0/front"
full_path=$full_path \
  docker-compose --file $full_path/docker-compose.yml up --build
