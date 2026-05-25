const os = require('os');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const packageJson = require('./package.json');
const VERSION = 'v' + packageJson.version; // Dynamically matches package version
const REPO = 'pd241008/DevTrace';

// Allow overriding the download URL for testing
const DOWNLOAD_URL = process.env.DEVTRACE_BINARY_URL || `https://github.com/${REPO}/releases/download/${VERSION}`;

const platform = os.platform();
const arch = os.arch();

// Map Node.js platform and arch to our release binary naming convention
const getBinaryName = () => {
  let binPlatform = platform;
  let binArch = arch;
  let ext = '';

  if (platform === 'win32') {
    ext = '.exe';
  } else if (platform === 'darwin') {
    binPlatform = 'macos';
  }

  // We expect binaries named like: devtrace-linux-x64, devtrace-macos-arm64, devtrace-win32-x64.exe
  return `devtrace-${binPlatform}-${binArch}${ext}`;
};

const binaryName = getBinaryName();
const url = `${DOWNLOAD_URL}/${binaryName}`;
const binDir = path.join(__dirname, 'bin');
const destFile = path.join(binDir, platform === 'win32' ? 'devtrace.exe' : 'devtrace');

async function downloadBinary() {
  console.log(`[DevTrace] Detecting platform: ${platform} (${arch})`);
  console.log(`[DevTrace] Downloading binary from: ${url}`);

  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  try {
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`Failed to download binary: ${response.status} ${response.statusText}`);
    }

    const buffer = await response.arrayBuffer();
    fs.writeFileSync(destFile, Buffer.from(buffer));

    // Make the binary executable on Unix-like systems
    if (platform !== 'win32') {
      execSync(`chmod +x "${destFile}"`);
    }

    console.log(`[DevTrace] Successfully downloaded to ${destFile}`);
  } catch (error) {
    console.error(`[DevTrace] Error downloading binary:`, error.message);
    console.error(`[DevTrace] Note: If the release is not published yet, you must build from source or set DEVTRACE_BINARY_URL to a valid location.`);
    process.exit(1);
  }
}

downloadBinary();
