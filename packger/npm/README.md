# DevTrace Node.js / Express Wrapper

This is the Node.js wrapper for **DevTrace** — the Distributed Developer Observability Engine.

This package automatically downloads and installs the high-performance Rust proxy engine tailored for your operating system and architecture during `npm install`.

## Requirements

- Node.js v18 or higher
- An internet connection (during installation) to fetch the pre-built Rust binaries.

## Installation

```bash
npm install devtrace
```

*Note: The postinstall script will automatically download the correct binary (`devtrace-linux-x64`, `devtrace-macos-arm64`, etc.) from the DevTrace GitHub releases.*

## Usage in Node.js / Express

You can programmatically start the DevTrace proxy from within your Node.js application.

```javascript
const { DevTrace } = require('devtrace');

// 1. Initialize the DevTrace Wrapper
const devtrace = new DevTrace({
  env: {
    // Pass custom environment variables if needed
    RUST_LOG: 'info'
  }
});

// 2. Start the Rust Proxy Server (Background Process)
devtrace.start();

// 3. Your Express Server Setup
const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.send('Hello from Express');
});

// DevTrace proxy will be running on port 8080 by default.
// Ensure your client/frontend routes its API requests through the proxy!
app.listen(3000, () => {
  console.log('Express app listening on port 3000');
});
```

## Replaying Requests

You can also trigger replays programmatically:

```javascript
// Replay request ID 42
devtrace.replay(42)
  .then(() => console.log('Replay finished'))
  .catch((err) => console.error(err));
```

## Development & Troubleshooting

If you need to install the package without downloading from GitHub (e.g., local development), you can set the `DEVTRACE_BINARY_URL` environment variable to a custom HTTP endpoint before running `npm install`.

```bash
export DEVTRACE_BINARY_URL="http://localhost:8000/my-local-builds"
npm install
```

If the binary fails to install, ensure that your OS and Architecture are supported in our [GitHub Releases](https://github.com/pd241008/DevTrace/releases).
