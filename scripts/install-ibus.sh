#!/bin/bash
set -euo pipefail

echo "Building avro-ibus in release mode..."
cargo build --release -p avro-ibus

echo "Installing binary to /usr/local/bin/avro-ibus..."
sudo cp target/release/avro-ibus /usr/local/bin/avro-ibus
sudo chmod +x /usr/local/bin/avro-ibus

echo "Installing component XML to /usr/share/ibus/component/avro-rust.xml..."
sudo mkdir -p /usr/share/ibus/component
sudo cp avro-rust.xml /usr/share/ibus/component/avro-rust.xml

echo "Restarting IBus daemon..."
ibus restart || echo "Could not restart IBus. Please restart it manually or log out."

echo "Installation complete!"
