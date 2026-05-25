#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

const binaryPath = path.join(__dirname, 'bin', os.platform() === 'win32' ? 'devtrace.exe' : 'devtrace');

if (!fs.existsSync(binaryPath)) {
  console.error(`[DevTrace] Error: Binary not found at ${binaryPath}`);
  console.error('[DevTrace] Did the installation fail? Try running npm install again.');
  process.exit(1);
}

// Pass all arguments after "node cli.js" down to the rust binary
const args = process.argv.slice(2);

// If no arguments were passed, default to serve or show help
if (args.length === 0) {
  args.push('serve');
}

console.log(`[DevTrace CLI] Running: devtrace ${args.join(' ')}`);

const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  env: process.env
});

child.on('error', (err) => {
  console.error('[DevTrace CLI] Failed to execute binary:', err);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code || 0);
});
