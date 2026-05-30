# DevTrace Python Wrapper

This is the Python wrapper for **DevTrace** — the Distributed Developer Observability Engine.

This package dynamically manages the high-performance Rust proxy engine for you. Upon first execution, it automatically downloads the correct binary tailored for your OS and architecture directly from GitHub Releases, meaning you don't need to manually compile Rust or manage paths!

## Installation

```bash
pip install devtrace
```

## CLI Usage

You can run the proxy server easily from your terminal.

```bash
# Start the proxy server
devtrace serve
```

Or trigger a replay by ID:
```bash
devtrace replay 42
```

## Programmatic Usage (Flask, FastAPI, Django)

You can embed and control the DevTrace proxy directly inside your Python backend applications.

```python
from devtrace import DevTrace
import time

# Initialize the proxy (you can optionally pass env vars)
proxy = DevTrace(env={"RUST_LOG": "info"})

# Start the DevTrace Engine in the background.
# This will download the binary on the first run automatically!
proxy.start()

print("Proxy is running! Your Python app can now route traffic through it.")

# (Start your Flask/FastAPI server here...)
# For demonstration, we'll just sleep
try:
    time.sleep(600)
except KeyboardInterrupt:
    pass
finally:
    # Make sure to clean up the proxy process
    proxy.stop()
```

## Replaying Requests from Python

You can trigger replays programmatically for automated testing or debugging scripts:

```python
from devtrace import DevTrace

proxy = DevTrace()
# Replay request ID 42
proxy.replay(42)
```

## Custom Binary Location

If you need to use a custom binary download server (e.g. for internal enterprise hosting), set the `DEVTRACE_BINARY_URL` environment variable before the proxy is executed for the first time.

```bash
export DEVTRACE_BINARY_URL="http://localhost:8000/my-local-builds"
devtrace serve
```
