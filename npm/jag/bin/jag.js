#!/usr/bin/env node
'use strict';

// The launcher of jag for npm. It finds the binary package of this platform
// and starts the binary. It uses no network module. The spec is
// docs/superpowers/specs/2026-10-14-STORY-48-install-jag-from-github-releases-crates-io-or-npm-design.md,
// section "The launcher".

const { spawn } = require('node:child_process');
const os = require('node:os');

// The four platforms. A test checks that this table agrees with package.json.
const PACKAGES = {
  'linux x64': '@airf/jag-linux-x64',
  'linux arm64': '@airf/jag-linux-arm64',
  'darwin arm64': '@airf/jag-darwin-arm64',
  'darwin x64': '@airf/jag-darwin-x64',
};

const FORWARDED = ['SIGINT', 'SIGTERM', 'SIGHUP', 'SIGQUIT'];

const messages = {
  unsupported: (platform, arch) =>
    `jag: npm has no jag binary for ${platform} ${arch}. jag runs on Linux x64 and arm64, and on macOS arm64 and x64. To build jag from source, run: cargo install --locked jag-grep`,
  notInstalled: (name) =>
    `jag: the package ${name} is not installed. npm does not install it with --omit=optional or --no-optional. Install @airf/jag again without these options.`,
  cannotStart: (path, code) => `jag: cannot start ${path}: ${code}. Install @airf/jag again.`,
};

function packageName(platform, arch) {
  return PACKAGES[`${platform} ${arch}`] || null;
}

// Returns { path } or { code, message }. resolve is require.resolve, so a test can replace it.
function findBinary(platform, arch, resolve) {
  const name = packageName(platform, arch);
  if (name === null) {
    return { code: 127, message: messages.unsupported(platform, arch) };
  }
  try {
    return { path: resolve(`${name}/bin/jag`) };
  } catch (error) {
    return { code: 127, message: messages.notInstalled(name) };
  }
}

function signalNumber(signal) {
  return os.constants.signals[signal];
}

function run() {
  const found = findBinary(process.platform, process.arch, require.resolve);
  if (found.message) {
    process.stderr.write(`${found.message}\n`);
    process.exitCode = found.code;
    return;
  }

  const child = spawn(found.path, process.argv.slice(2), { stdio: 'inherit' });

  const listeners = FORWARDED.map((signal) => {
    const listener = () => {
      try {
        child.kill(signal);
      } catch (error) {
        // The child has ended already.
      }
    };
    process.on(signal, listener);
    return [signal, listener];
  });
  const removeListeners = () => {
    for (const [signal, listener] of listeners) {
      process.removeListener(signal, listener);
    }
  };

  child.on('error', (error) => {
    removeListeners();
    process.stderr.write(`${messages.cannotStart(found.path, error.code)}\n`);
    process.exitCode = error.code === 'ENOENT' ? 127 : 126;
  });

  child.on('exit', (code, signal) => {
    removeListeners();
    if (signal === null) {
      process.exitCode = code;
      return;
    }
    // Node ignores SIGPIPE, so the launcher cannot end by it.
    if (signal === 'SIGPIPE') {
      process.exitCode = 128 + signalNumber(signal);
      return;
    }
    // End by the same signal. When the signal does not end the launcher, exit with 128 + n.
    process.exitCode = 128 + signalNumber(signal);
    process.kill(process.pid, signal);
  });
}

if (require.main === module) {
  run();
}

module.exports = { packageName, messages, findBinary };
