# Event Monitor

This tool monitors a Stellar smart contract for anomalous on-chain activity and sends alerts to a webhook.

## Features

*   Connects to the Horizon Streaming Service (SSE) to receive contract events in real-time.
*   Monitors for the following anomalous events:
    *   Unusual fund release patterns
    *   High-volume donations
    *   Contract freeze events
*   Sends alerts to a PagerDuty or Slack webhook.

## Building and Running

1.  **Build the tool:**

    ```bash
    cargo build --release -p event-monitor
    ```

2.  **Configure the environment:**

    The following environment variables must be set:

    *   `CONTRACT_ID`: The ID of the contract to monitor.
    *   `WEBHOOK_URL`: The URL of the webhook to send alerts to.

3.  **Run the tool:**

    ```bash
    ./target/release/event-monitor
    ```

## Deployment

For continuous operation, it is recommended to run the `event-monitor` as a systemd service or in a container.

### systemd Service

1.  Create a new service file at `/etc/systemd/system/event-monitor.service`:

    ```ini
    [Unit]
    Description=StellarAid Event Monitor
    After=network.target

    [Service]
    User=your-user
    Group=your-group
    WorkingDirectory=/path/to/stellarAid-contract
    Environment="CONTRACT_ID=your-contract-id"
    Environment="WEBHOOK_URL=your-webhook-url"
    ExecStart=/path/to/stellarAid-contract/target/release/event-monitor
    Restart=always

    [Install]
    WantedBy=multi-user.target
    ```

2.  Reload the systemd daemon:

    ```bash
    sudo systemctl daemon-reload
    ```

3.  Enable and start the service:

    ```bash
    sudo systemctl enable event-monitor
    sudo systemctl start event-monitor
    ```

### Docker Container

A `Dockerfile` is not yet provided, but you can create one to containerize the `event-monitor`.