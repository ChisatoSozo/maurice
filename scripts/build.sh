#!/bin/bash

#set -e  # Exit immediately if a command exits with a non-zero status.

lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true

# Run the initial commands
python functions/python/type_gen.py
cargo typify functions/function_schemas.json --additional-derive Apiv2Schema --output server/src/function_types/types.rs
sed -i 's/use serde::{Deserialize, Serialize};/use paperclip::actix::Apiv2Schema;\nuse serde::{Deserialize, Serialize};/' server/src/function_types/types.rs

# Modify for no tuples
sed -E '
s/([A-Z][a-zA-Z0-9]*)\(pub (\(\)|[^)]+)\);/\1{value: \2}/g;
s/\{value: \(\)\}/\{value: ()\}/g
' -i server/src/function_types/types.rs
sed -E 's/\.0/.value/g' -i server/src/function_types/types.rs
sed -E 's/Self\(([^.]+)\.parse\(\)\?\)/Self{value: \1.parse()?}/g' -i server/src/function_types/types.rs
sed -E 's/Self\(([^.]+)\.to_string\(\)\)/Self{value: \1.to_string()}/g' -i server/src/function_types/types.rs
sed -E 's/Self\(([^.]+)\.into\(\)\)/Self{value: \1.into()}/g' -i server/src/function_types/types.rs
sed -E 's/Self\(([^)]+)\)/Self{value: \1}/g' -i server/src/function_types/types.rs
sed -E '
/^pub mod error \{/,/^\}/ {
  /std::fmt::Display::fmt\(&self\.value, f\)/s/value/0/
  /std::fmt::Debug::fmt\(&self\.value, f\)/s/value/0/
  /Self\{value: value\.into\(\)\}/s/\{value: (.*)\.into\(\)\}/(\1.into())/
}
' -i server/src/function_types/types.rs

# Function to start the server
start_server() {
    cd server
    cargo run > server.log 2>&1 &
    echo $! > server.pid
    cd ..
}

# Function to stop the server
stop_server() {
    if [ -f server/server.pid ]; then
        PID=$(cat server/server.pid)
        echo "Stopping server (PID: $PID)..."
        kill -TERM $PID 2>/dev/null || true
        sleep 2
        if kill -0 $PID 2>/dev/null; then
            echo "Server didn't stop gracefully. Forcing shutdown..."
            kill -KILL $PID 2>/dev/null || true
        fi
        rm server/server.pid
        echo "Server log:"
        cat server/server.log
    else
        echo "No server PID file found."
    fi
}

# Function to check if the server is available
check_server() {
    curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/spec/v2.json
}

# Trap to ensure server is stopped on script exit
trap stop_server EXIT

# Start the server
echo "Starting server..."
start_server

# Wait for the server to become available
echo "Waiting for server to start..."
TIMEOUT=60
ELAPSED=0
while true; do
    HTTP_STATUS=$(check_server)
    if [ "$HTTP_STATUS" = "200" ]; then
        echo "Server is up and running!"
        break
    fi
    if [ $ELAPSED -ge $TIMEOUT ]; then
        echo "Timeout waiting for server to start. Exiting."
        exit 1
    fi
    sleep 1
    ELAPSED=$((ELAPSED + 1))
done

# Run yarn gen
cd web-client && yarn gen

lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true

echo "Build process completed."