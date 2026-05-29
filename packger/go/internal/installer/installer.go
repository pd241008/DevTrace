package installer

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
)

const (
	Version = "v0.1.1"
	Repo    = "pd241008/DevTrace"
)

// EnsureBinary checks if the binary is installed in ~/.devtrace/bin.
// If not, it downloads the correct binary for the current OS/Arch.
func EnsureBinary() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get home directory: %w", err)
	}

	binDir := filepath.Join(home, ".devtrace", "bin")
	if err := os.MkdirAll(binDir, 0755); err != nil {
		return "", fmt.Errorf("failed to create bin directory: %w", err)
	}

	binName := "devtrace"
	if runtime.GOOS == "windows" {
		binName = "devtrace.exe"
	}

	binPath := filepath.Join(binDir, binName)
	if _, err := os.Stat(binPath); err == nil {
		// Binary already exists
		return binPath, nil
	}

	// Not found, download it
	remoteBinName := getRemoteBinaryName()
	downloadURL := getDownloadURL(remoteBinName)

	fmt.Printf("[DevTrace] Downloading binary for %s/%s from %s...\n", runtime.GOOS, runtime.GOARCH, downloadURL)

	if err := downloadFile(binPath, downloadURL); err != nil {
		return "", fmt.Errorf("failed to download binary: %w\nEnsure you have an internet connection and the release exists.", err)
	}

	// Make executable
	if runtime.GOOS != "windows" {
		if err := os.Chmod(binPath, 0755); err != nil {
			return "", fmt.Errorf("failed to make binary executable: %w", err)
		}
	}

	fmt.Printf("[DevTrace] Successfully installed to %s\n", binPath)
	return binPath, nil
}

func getRemoteBinaryName() string {
	osName := runtime.GOOS
	if osName == "darwin" {
		osName = "macos" // Match our release matrix naming
	}
	
	archName := runtime.GOARCH
	if archName == "amd64" {
		archName = "x64"
	}

	ext := ""
	if runtime.GOOS == "windows" {
		ext = ".exe"
	}

	return fmt.Sprintf("devtrace-%s-%s%s", osName, archName, ext)
}

func getDownloadURL(remoteBinName string) string {
	customURL := os.Getenv("DEVTRACE_BINARY_URL")
	if customURL != "" {
		return fmt.Sprintf("%s/%s", customURL, remoteBinName)
	}
	return fmt.Sprintf("https://github.com/%s/releases/download/%s/%s", Repo, Version, remoteBinName)
}

func downloadFile(filepath string, url string) error {
	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("bad status: %s", resp.Status)
	}

	_, err = io.Copy(out, resp.Body)
	return err
}
