#!/bin/bash

# # Check if a "funnel" container is already running
# if [ $(docker ps -q -f name=funnel) ]; then
#     # If it is, stop and remove it
#     docker stop funnel
#     docker rm funnel
# fi

# # Build and run the Dockerized Funnel server
# cd funnel/
# docker build -t funnel -f ./Dockerfile .
# docker run -d --name funnel -p 8000:8000 funnel
# cd ..

# Check if a "funnel" process is already running
if ! ps aux | grep '[f]unnel server run'; then
    # If it's not running, start it
    echo "Funnel server is not running. Starting it now..."
    funnel server run --Server.HostName=localhost --Server.HTTPPort=8000 > funnel.log 2>&1 &
else
    echo "Funnel server is already running."
fi

# Wait for the Funnel server to start
echo "Waiting for Funnel server to start..."
while ! curl -s http://localhost:8000/healthz > /dev/null
do
    echo "Waiting for Funnel server..."
    sleep 1
done
echo "Funnel server is running."

# Run the tests
cargo test

# # Stop and remove the Funnel server container
# docker stop funnel
# docker rm funnel