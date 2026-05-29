package main

import (
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"syscall"

	"github.com/pd241008/DevTrace/packger/go/internal/installer"
)

func main() {
	// First ensure the binary is downloaded for this platform
	binPath, err := installer.EnsureBinary()
	if err != nil {
		fmt.Fprintf(os.Stderr, "[DevTrace CLI] Error: %v\n", err)
		os.Exit(1)
	}

	args := os.Args[1:]
	if len(args) == 0 {
		args = []string{"serve"}
	}

	fmt.Printf("[DevTrace CLI] Running: devtrace %v\n", args)

	// Execute the rust binary, passing through all CLI arguments
	cmd := exec.Command(binPath, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	cmd.Env = os.Environ()

	if err := cmd.Start(); err != nil {
		fmt.Fprintf(os.Stderr, "[DevTrace CLI] Failed to execute binary: %v\n", err)
		os.Exit(1)
	}

	// Handle graceful shutdown via Ctrl+C
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sigChan
		if cmd.Process != nil {
			cmd.Process.Kill()
		}
	}()

	if err := cmd.Wait(); err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			os.Exit(exitError.ExitCode())
		}
		os.Exit(1)
	}
}
