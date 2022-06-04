#!/bin/bash
# $HOME/github.com/loicbourgois/tachikosmachines/wasm/_go.sh
DIR="$( cd "$( dirname "$0" )" && pwd )"
CONTAINER_DIR=$(echo $DIR | sed 's|'$HOME'|/root|g')
DIR=$DIR \
  CONTAINER_DIR=$CONTAINER_DIR \
  docker-compose --file $DIR/docker-compose.yml up --build \
  --exit-code-from wasm
