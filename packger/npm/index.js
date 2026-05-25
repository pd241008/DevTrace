const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

class DevTrace {
  /**
   * Initialize the DevTrace Engine wrapper
   * @param {Object} options 
   * @param {Object} options.env - Environment variables to pass to the Rust proxy
   */
  constructor(options = {}) {
    this.options = options;
    this.binaryPath = path.join(__dirname, 'bin', os.platform() === 'win32' ? 'devtrace.exe' : 'devtrace');
    this.process = null;

    if (!fs.existsSync(this.binaryPath)) {
      console.warn(`[DevTrace] Warning: Binary not found at ${this.binaryPath}. Did the postinstall script fail?`);
    }
  }

  /**
   * Start the DevTrace proxy server
   */
  start() {
    if (this.process) {
      console.warn('[DevTrace] Proxy is already running.');
      return;
    }

    console.log('[DevTrace] Starting proxy server...');
    this.process = spawn(this.binaryPath, ['serve'], {
      stdio: 'inherit', // Let the Rust logs pass through to the Node console
      env: { ...process.env, ...this.options.env }
    });

    this.process.on('error', (err) => {
      console.error('[DevTrace] Failed to start proxy:', err);
    });

    this.process.on('close', (code) => {
      if (code !== 0 && code !== null) {
        console.warn(`[DevTrace] Proxy exited with code ${code}`);
      }
      this.process = null;
    });

    // Make sure we kill the proxy when the Node process exits
    const cleanup = () => this.stop();
    
    process.on('exit', cleanup);
    process.on('SIGINT', () => { cleanup(); process.exit(); });
    process.on('SIGTERM', () => { cleanup(); process.exit(); });
  }

  /**
   * Stop the running DevTrace proxy server
   */
  stop() {
    if (this.process) {
      console.log('[DevTrace] Stopping proxy...');
      this.process.kill();
      this.process = null;
    }
  }

  /**
   * Fire a replay for a specific request ID directly from JS
   * @param {number|string} id 
   */
  replay(id) {
    console.log(`[DevTrace] Replaying request ID: ${id}`);
    const replayProcess = spawn(this.binaryPath, ['replay', String(id)], {
      stdio: 'inherit',
      env: { ...process.env, ...this.options.env }
    });

    return new Promise((resolve, reject) => {
      replayProcess.on('close', (code) => {
        if (code === 0) resolve();
        else reject(new Error(`Replay failed with code ${code}`));
      });
      replayProcess.on('error', reject);
    });
  }
}

module.exports = { DevTrace };
