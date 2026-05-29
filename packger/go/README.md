# DevTrace Go Wrapper

This is the Go wrapper for **DevTrace** — the Distributed Developer Observability Engine.

This package automatically downloads and manages the high-performance Rust proxy engine tailored for your operating system and architecture upon first execution, without requiring you to manually compile Rust or manage binaries!

## CLI Installation & Usage

You can install the DevTrace CLI tool globally via `go install`:

```bash
go install github.com/pd241008/DevTrace/packger/go/cmd/devtrace@latest
```

This will put the `devtrace` command in your `$GOPATH/bin` (make sure it's in your `$PATH`).

You can then run the proxy server easily from anywhere:
```bash
devtrace serve
```

Or trigger a replay by ID:
```bash
devtrace replay 42
```

## Programmatic Usage in Go Applications

You can embed and control the DevTrace proxy directly inside your Go backend services (like Gin, Echo, or standard `net/http` servers).

### 1. Add the Dependency
```bash
go get github.com/pd241008/DevTrace/packger/go
```

### 2. Start the Proxy Programmatically

```go
package main

import (
	"fmt"
	"time"

	"github.com/pd241008/DevTrace/packger/go"
)

func main() {
	// Initialize the proxy (you can optionally pass env vars)
	proxy := devtrace.New("RUST_LOG=info")

	// Start the DevTrace Engine in the background.
	// This will download the binary on the first run automatically!
	err := proxy.Start()
	if err != nil {
		panic(err)
	}

	// Make sure to clean up the proxy process when your Go app exits
	defer proxy.Stop()

	fmt.Println("Proxy is running! Your Go app can now route traffic through it.")
	
	// Keep the main thread alive (or start your web server here)
	time.Sleep(10 * time.Minute)
}
```

## Replaying Requests from Go

You can trigger replays programmatically for automated testing or debugging scripts:

```go
// Replay request ID 42
err := proxy.Replay(42)
if err != nil {
	fmt.Printf("Replay failed: %v\n", err)
}
```

## Development & Custom Binary Location

If you need to use a custom binary download server (for internal enterprise hosting or local development), set the `DEVTRACE_BINARY_URL` environment variable.

```bash
export DEVTRACE_BINARY_URL="http://localhost:8000/my-local-builds"
go run ./cmd/devtrace serve
```
