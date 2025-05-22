# Kaspa Monitor

A command-line tool for monitoring Kaspa nodes, checking their health status and connection latency.

[![GitHub release](https://img.shields.io/github/v/release/forbole/kaspa-monitor)](https://github.com/forbole/kaspa-monitor/releases/latest)
[![License](https://img.shields.io/github/license/forbole/kaspa-monitor)](https://github.com/forbole/kaspa-monitor/blob/main/LICENSE)

## Features

- Check node health and sync status
- Test connection latency
- Support for multiple network types
- Automatic node discovery via resolver (if no endpoint is specified)

## Installation

### Download Pre-built Binaries

Pre-built binaries for Linux, macOS, and Windows are available on the [Releases](https://github.com/forbole/kaspa-monitor/releases) page.

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/forbole/kaspa-monitor.git
cd kaspa-monitor
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/kaspa-monitor`

## Usage

### Command Line Options

```
Kaspa Node Monitoring Tool

Usage: kaspa-monitor [OPTIONS] <COMMAND>

Commands:
  healthcheck    Check node health and sync status
  latencycheck   Test connection latency
  help           Print this message or the help of the given subcommand(s)

Options:
  -e, --endpoint <ENDPOINT>  Kaspa node WebSocket endpoint
  -h, --help                 Print help
  -V, --version              Print version
```

### Examples

#### Health Check

Check the health of a specific Kaspa node:

```bash
kaspa-monitor healthcheck --endpoint wss://kaspa.example.com/ws
```

Or let the resolver automatically find a node:

```bash
kaspa-monitor healthcheck
```

#### Latency Check

Test the connection latency to a specific node:

```bash
kaspa-monitor latencycheck --endpoint wss://kaspa.example.com/ws
```

### Example Output

#### Health Check Output

```
Initializing connection test...
Endpoint: wss://kaspa.example.com/ws
Client creation took: 0.002s

Connection Results:
Connection establishment time: 0.128s
Total initialization time: 0.131s

Fetching sync status...

Sync Status:
SyncStatus {
    is_synced: true,
    virtual_daa_score: 12345678,
    virtual_parent_blue_score: 87654321,
    header_count: 654321,
    block_count: 543210,
    connected_peers_count: 15,
    total_syncer_down_load_item: 0,
    states: {
        "StateInvalid": 0,
        "StateReconciling": 0,
        "StateUTXOValidation": 0,
        "StateTrustedData": 0,
        "StateHeaderPruning": 0,
        "StateSelectingTips": 0,
        "StateVirtual": 9856,
    },
}
```

#### Latency Check Output

```
Initializing connection test...
Endpoint: wss://kaspa.example.com/ws
Client creation took: 0.003s

Connection Results:
Connection establishment time: 0.142s
Total initialization time: 0.145s
```

## Testing

To run the test suite:

```bash
cargo test
```

## Use Cases

- Monitoring node health in production environments
- Checking node sync status for validators
- Testing connection latency to different nodes
- Validating node setup during deployment

## Troubleshooting

### Connection Issues

If you encounter connection issues:

1. Verify the node endpoint is correct and accessible
2. Check if the node's WebSocket service is running
3. Ensure no firewall is blocking the connection
4. Try with the automatic resolver by omitting the endpoint

### Error Messages

- "Connection timeout after 5 seconds!": The node is not responding within the timeout period
- "Unable to connect to the node": The connection was refused or failed

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Bash Scripting Usage

kaspa-monitor is designed to be friendly for use in shell scripts with consistent exit codes.

### Exit Codes

The following exit codes are used:

| Exit Code | Description |
|-----------|-------------|
| 0         | Success - The command completed successfully |
| 1         | General Error - A non-specific error occurred |
| 2         | Connection Error - Failed to connect to the Kaspa node |
| 3         | Timeout Error - Connection to the node timed out |
| 4         | Client Error - Error initializing the client |
| 5         | RPC Error - Error during RPC communication |

### Scripting Examples

#### Check if a node is synced:

```bash
#!/bin/bash

kaspa-monitor healthcheck --endpoint wss://kaspa.example.com/ws
if [ $? -eq 0 ]; then
  echo "Node is healthy"
  exit 0
else
  echo "Node health check failed"
  exit 1
fi
```

#### Monitor multiple nodes:

```bash
#!/bin/bash

NODES=("wss://node1.example.com/ws" "wss://node2.example.com/ws" "wss://node3.example.com/ws")
HEALTHY_NODES=0

for node in "${NODES[@]}"; do
  echo "Checking $node..."
  kaspa-monitor healthcheck --endpoint $node
  if [ $? -eq 0 ]; then
    echo "$node is healthy"
    HEALTHY_NODES=$((HEALTHY_NODES+1))
  else
    echo "$node has issues"
  fi
done

echo "$HEALTHY_NODES/${#NODES[@]} nodes are healthy"

if [ $HEALTHY_NODES -eq ${#NODES[@]} ]; then
  echo "All nodes are healthy"
  exit 0
else
  echo "Some nodes have issues"
  exit 1
fi
```
