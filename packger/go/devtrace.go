package devtrace

import (
	"fmt"
	"os"
	"os/exec"
	"strconv"

	"github.com/pd241008/DevTrace/packger/go/internal/installer"
)

// DevTrace represents the proxy engine wrapper.
type DevTrace struct {
	cmd     *exec.Cmd
	env     []string
}

// New creates a new DevTrace instance.
// You can pass optional environment variables in the format "KEY=VALUE".
func New(env ...string) *DevTrace {
	return &DevTrace{
		env: env,
	}
}

// Start ensures the binary is downloaded and starts the proxy server in the background.
func (d *DevTrace) Start() error {
	if d.cmd != nil {
		return fmt.Errorf("[DevTrace] Proxy is already running")
	}

	binPath, err := installer.EnsureBinary()
	if err != nil {
		return err
	}

	d.cmd = exec.Command(binPath, "serve")
	d.cmd.Stdout = os.Stdout
	d.cmd.Stderr = os.Stderr
	
	// Inherit current environment and append custom env vars
	d.cmd.Env = os.Environ()
	d.cmd.Env = append(d.cmd.Env, d.env...)

	fmt.Println("[DevTrace] Starting proxy server...")
	if err := d.cmd.Start(); err != nil {
		return fmt.Errorf("failed to start proxy: %w", err)
	}

	// Spin up a goroutine to wait for the command to finish so it doesn't become a zombie
	go func() {
		if err := d.cmd.Wait(); err != nil {
			fmt.Printf("[DevTrace] Proxy exited: %v\n", err)
		}
		d.cmd = nil
	}()

	return nil
}

// Stop terminates the proxy server if it is running.
func (d *DevTrace) Stop() error {
	if d.cmd == nil || d.cmd.Process == nil {
		return nil
	}
	
	fmt.Println("[DevTrace] Stopping proxy...")
	err := d.cmd.Process.Kill()
	if err != nil {
		return fmt.Errorf("failed to kill proxy process: %w", err)
	}
	d.cmd = nil
	return nil
}

// Replay fires a replay for a specific request ID directly.
func (d *DevTrace) Replay(id uint64) error {
	binPath, err := installer.EnsureBinary()
	if err != nil {
		return err
	}

	fmt.Printf("[DevTrace] Replaying request ID: %d\n", id)
	cmd := exec.Command(binPath, "replay", strconv.FormatUint(id, 10))
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	
	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, d.env...)

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("replay failed: %w", err)
	}
	return nil
}
