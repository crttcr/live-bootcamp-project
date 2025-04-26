#!/bin/bash

# Define the location of the .env file (change if needed)
ENV_FILE="./auth-service/.env"
export COMPOSE_BAKE=true

# Check if the .env file exists
if ! [[ -f "$ENV_FILE" ]]; then
  echo "Error: .env file not found!"
  exit 1
fi

# Read each line in the .env file (ignoring comments)
while IFS= read -r line; do
  if [[ -n "$line" ]] && [[ "$line" != \#* ]]; then   # Skip blank lines and lines starting with #
    k=$(echo "$line" | cut -d '=' -f1)                # Split the line into key and value
    v=$(echo "$line" | cut -d '=' -f2-)
    export "$k=$v"                                    # Export the variable
  fi
done < <(grep -v '^#' "$ENV_FILE")

# Run docker-compose commands with exported variables
docker-compose build
docker-compose up

