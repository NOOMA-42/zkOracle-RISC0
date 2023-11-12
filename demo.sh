#!/bin/bash


# Check if port is already in use
# kill $(lsof -t -i:$PORT)

# Check if at least one argument was provided
if [ $# -lt 2 ]; then
    echo "Usage: $0 <bonsai> <devmode>..."
    echo "Argument should be either T or F"
    exit 1
fi

# Bonsai or not
if [ "$1" = "T" ]; then
    export BONSAI_API_URL="https://api.bonsai.xyz/"
    export BONSAI_API_KEY="AdiM1hmFnyaNMt3dZQPSr6xMfiZvISQZ6hQASSTt"
elif [ "$1" = "F" ]; then
    export BONSAI_API_URL=""
    export BONSAI_API_KEY=""
else
    echo "error: invalid argument: $1"
    exit 
fi

# Devmode or not
if [ "$2" = "T" ]; then
    export DEVMODE=true
elif [ "$2" = "F" ]; then
    export DEVMODE=false
else
    echo "error: invalid argument: $1"
    exit
fi

# Setup
cd simple-proxy
yarn 

# Init simple-proxy server
yarn start &

# Execute oracle
cd ..
cargo run