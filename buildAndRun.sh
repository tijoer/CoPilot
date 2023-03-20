#!/bin/bash

# end on error
set -e

# trap ctrl_c and call ctrl_c()
function ctrl_c() {
    # uft-8 red x
    echo -e "\e[31m\u2717\e[0m Aborting..."
    docker stop myapp
    docker rm myapp
    exit 1
}
trap ctrl_c INT

# kill all running containers (if any)
docker kill $(docker ps -q) > /dev/null 2>&1 || true

# remove all containers (if any)
docker rm $(docker ps -a -q) > /dev/null 2>&1 || true

# build the docker image
docker build -t myapp .

# run the docker image and mount the html folder
docker run -d --name myapp -p 8080:80 -v $(pwd)/html:/usr/share/nginx/html myapp



#docker run -d --name myapp -p 8080:80 myapp 
