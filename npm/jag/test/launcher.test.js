'use strict';

// Tests of the launcher. Run: node --test npm/jag/test/launcher.test.js
// They use the node:test API of Node 18. A fake binary package in a
// temporary node_modules stands in for the real binary.

const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');

const launcherPath = path.join(__dirname, '..', 'bin', 'jag.js');
const launcher = require(launcherPath);
const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'));

const here = launcher.packageName(process.platform, process.arch);

const roots = [];
process.on('exit', () => {
  for (const root of roots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

// A temporary install: node_modules/@airf/jag/bin/jag.js and a fake binary package.
function install(script, { mode = 0o755, withBinary = true } = {}) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'jag-launcher-'));
  roots.push(root);
  const scope = path.join(root, 'node_modules', '@airf');
  const bin = path.join(scope, 'jag', 'bin');
  fs.mkdirSync(bin, { recursive: true });
  fs.copyFileSync(launcherPath, path.join(bin, 'jag.js'));
  if (withBinary) {
    const binaryDir = path.join(root, 'node_modules', here, 'bin');
    fs.mkdirSync(binaryDir, { recursive: true });
    fs.writeFileSync(path.join(binaryDir, 'jag'), script, { mode });
    fs.chmodSync(path.join(binaryDir, 'jag'), mode);
  }
  return { root, launcher: path.join(bin, 'jag.js') };
}

function runLauncher(installed, args, options = {}) {
  return spawnSync(process.execPath, [installed.launcher, ...args], { encoding: 'utf8', ...options });
}

test('the name for each of the four platforms', () => {
  assert.strictEqual(launcher.packageName('linux', 'x64'), '@airf/jag-linux-x64');
  assert.strictEqual(launcher.packageName('linux', 'arm64'), '@airf/jag-linux-arm64');
  assert.strictEqual(launcher.packageName('darwin', 'arm64'), '@airf/jag-darwin-arm64');
  assert.strictEqual(launcher.packageName('darwin', 'x64'), '@airf/jag-darwin-x64');
});

test('an unknown platform gives 127 and the message', () => {
  assert.strictEqual(launcher.packageName('win32', 'x64'), null);
  const found = launcher.findBinary('win32', 'x64', () => {
    throw new Error('must not resolve');
  });
  assert.strictEqual(found.code, 127);
  assert.strictEqual(found.message, launcher.messages.unsupported('win32', 'x64'));
  assert.match(found.message, /cargo install --locked jag-grep/);
});

test('the table of the launcher agrees with package.json', () => {
  const dependencies = Object.keys(packageJson.optionalDependencies).sort();
  const names = [];
  for (const [platform, arch] of [['linux', 'x64'], ['linux', 'arm64'], ['darwin', 'arm64'], ['darwin', 'x64']]) {
    const name = launcher.packageName(platform, arch);
    assert.strictEqual(name, `@airf/jag-${platform}-${arch}`);
    names.push(name);
  }
  assert.deepStrictEqual(names.sort(), dependencies);
});

test('a missing package gives 127', () => {
  const installed = install('', { withBinary: false });
  const result = runLauncher(installed, ['--version']);
  assert.strictEqual(result.status, 127);
  assert.strictEqual(result.stderr, `${launcher.messages.notInstalled(here)}\n`);
  assert.strictEqual(result.stdout, '');
});

test('the arguments reach the binary as they are', () => {
  const installed = install('#!/bin/sh\nprintf "[%s]\\n" "$@"\n');
  const result = runLauncher(installed, ['a b', '', '--', '-x', 'c"d']);
  assert.strictEqual(result.status, 0);
  assert.strictEqual(result.stdout, '[a b]\n[]\n[--]\n[-x]\n[c"d]\n');
});

test('the exit code of the binary is the exit code of the launcher', () => {
  for (const code of [0, 1, 2, 7, 130]) {
    const installed = install(`#!/bin/sh\nexit ${code}\n`);
    assert.strictEqual(runLauncher(installed, []).status, code, `exit ${code}`);
  }
});

test('the binary gets the standard input, and writes to its own streams', () => {
  const installed = install('#!/bin/sh\ncat\necho err >&2\n');
  const result = runLauncher(installed, [], { input: 'from stdin\n' });
  assert.strictEqual(result.stdout, 'from stdin\n');
  assert.strictEqual(result.stderr, 'err\n');
});

test('an end of the binary by SIGINT ends the launcher by SIGINT', () => {
  const installed = install('#!/bin/sh\nkill -INT $$\n');
  const result = runLauncher(installed, []);
  assert.strictEqual(result.signal, 'SIGINT');
});

test('an end of the binary by SIGPIPE ends the launcher by SIGPIPE or with 141', () => {
  const installed = install('#!/bin/sh\nkill -PIPE $$\n');
  const result = runLauncher(installed, []);
  assert.ok(result.signal === 'SIGPIPE' || result.status === 141, `status ${result.status}, signal ${result.signal}`);
});

test('an end of the binary by SIGUSR1 gives 128 + n and does not start the inspector', () => {
  const installed = install('#!/bin/sh\nkill -USR1 $$\n');
  const result = runLauncher(installed, []);
  assert.strictEqual(result.signal, null);
  assert.strictEqual(result.status, 128 + os.constants.signals.SIGUSR1);
  assert.doesNotMatch(result.stderr, /Debugger listening/);
});

test('SIGTERM to the launcher reaches the binary', (t, done) => {
  const installed = install(
    '#!/bin/sh\ntrap \'echo got-term; exit 3\' TERM\necho ready\nwhile :; do sleep 0.1; done\n',
  );
  const child = spawn(process.execPath, [installed.launcher], { stdio: ['ignore', 'pipe', 'inherit'] });
  let output = '';
  let sent = false;
  child.stdout.on('data', (chunk) => {
    output += chunk;
    if (!sent && output.includes('ready')) {
      sent = true;
      child.kill('SIGTERM');
    }
  });
  child.on('close', (code) => {
    try {
      assert.match(output, /got-term/);
      assert.strictEqual(code, 3);
      done();
    } catch (error) {
      done(error);
    }
  });
});

test('a binary with no execute bit gives 126', () => {
  const installed = install('#!/bin/sh\nexit 0\n', { mode: 0o644 });
  const result = runLauncher(installed, []);
  assert.strictEqual(result.status, 126);
  assert.match(result.stderr, /^jag: cannot start .*: EACCES\. Install @airf\/jag again\.\n$/);
});

test('the launcher loads no network module and has no install script', () => {
  const source = fs.readFileSync(launcherPath, 'utf8');
  for (const name of ['net', 'http', 'https', 'http2', 'dns', 'tls', 'dgram']) {
    assert.ok(!source.includes(`'node:${name}'`) && !source.includes(`'${name}'`), name);
  }
  for (const script of ['preinstall', 'install', 'postinstall', 'prepare']) {
    assert.ok(!(packageJson.scripts && script in packageJson.scripts), script);
  }
});
