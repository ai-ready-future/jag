'use strict';

// The install test of the jag agent skill (FEATURE-24, CRITERION-116).
// Run: node --test tests/agent_skill/install.test.js
//
// It runs the pinned `skills` tool on this checkout, not on GitHub, so that
// it tests the pull request. The tool writes only in temporary folders: the
// project folder and the home folder are new, and the home folder is the
// HOME of the tool. The test never touches the real agent folders of the
// machine. It makes no call to the model service.

const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

// A new pin of the tool needs a new file with the agent list (see below).
const SKILLS_VERSION = '1.7.4';

const checkout = path.join(__dirname, '..', '..');
const skillSource = path.join(checkout, 'skills', 'jag', 'SKILL.md');
const skillText = fs.readFileSync(skillSource);

const roots = [];
process.on('exit', () => {
  for (const root of roots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

function temporaryFolder(name) {
  const folder = fs.mkdtempSync(path.join(os.tmpdir(), `jag-skill-${name}-`));
  roots.push(folder);
  return folder;
}

// The home folder is shared by the tests, so that `npx` keeps the tool in its
// cache after the first run.
const home = temporaryFolder('home');

// The agents of the pinned tool and the project folder of each. The file is
// the table of docs/agents.md in the package skills@1.7.4.
function agentFolders() {
  const file = path.join(__dirname, `agents-${SKILLS_VERSION}.txt`);
  const folders = new Map();
  for (const line of fs.readFileSync(file, 'utf8').split('\n')) {
    if (line === '' || line.startsWith('#')) {
      continue;
    }
    const [folder, ids] = line.split('\t');
    folders.set(folder, [...(folders.get(folder) || []), ...ids.split(',')]);
  }
  return folders;
}

function skills(project, args) {
  const result = spawnSync('npx', ['--yes', `skills@${SKILLS_VERSION}`, ...args], {
    cwd: project,
    // No question may wait for a person: standard input is closed.
    stdio: ['ignore', 'pipe', 'pipe'],
    encoding: 'utf8',
    timeout: 240000,
    env: {
      ...process.env,
      HOME: home,
      USERPROFILE: home,
      DO_NOT_TRACK: '1',
      NO_COLOR: '1',
    },
  });
  assert.strictEqual(result.error, undefined, `skills did not run: ${result.error}`);
  return result;
}

// The `--json` output is one JSON array. The tool can write text before it.
function jsonOf(result) {
  const text = result.stdout;
  const start = text.startsWith('[') ? 0 : text.lastIndexOf('\n[') + 1;
  assert.ok(start >= 0 && text.includes('['), `no JSON in the output of skills:\n${text}`);
  return JSON.parse(text.slice(start));
}

function sameBytes(file) {
  assert.ok(fs.existsSync(file), `missing: ${file}`);
  assert.ok(fs.readFileSync(file).equals(skillText), `not the skill file: ${file}`);
}

// The text after the frontmatter, without the empty line after it.
function bodyOf(text) {
  const match = /^---\r?\n[\s\S]*?\r?\n---\r?\n(?:\r?\n)?([\s\S]*)$/.exec(text);
  assert.ok(match, 'the file has no frontmatter');
  return match[1];
}

// The folder of Eve gets a copy with the frontmatter written again by the
// tool. The body is the same, and the name and the description are the same.
function sameSkill(file) {
  assert.ok(fs.existsSync(file), `missing: ${file}`);
  const text = fs.readFileSync(file, 'utf8');
  assert.strictEqual(bodyOf(text), bodyOf(skillText.toString('utf8')), `body differs: ${file}`);
  assert.match(text, /^name: jag$/m);
  assert.match(text, /^description: ./m);
}

test('the folder skills holds one skill and nothing else', () => {
  assert.deepStrictEqual(fs.readdirSync(path.join(checkout, 'skills')), ['jag']);
  assert.deepStrictEqual(fs.readdirSync(path.join(checkout, 'skills', 'jag')), ['SKILL.md']);
});

// CRITERION-116: the skill is in each agent that the command supports.
test('the install command puts the skill in each agent of the pinned tool', () => {
  const project = temporaryFolder('project');
  const folders = agentFolders();
  assert.ok(folders.size > 50, 'the agent list is too short');
  // The tool puts a link in the folder of an agent only when the top folder
  // of that agent exists in the project (or the agent is the one that the
  // user names). So the project has the top folder of each agent, as the
  // project of a person who uses that agent has.
  for (const folder of folders.keys()) {
    fs.mkdirSync(path.join(project, folder.split('/')[0]), { recursive: true });
  }

  const added = skills(project, ['add', checkout, '--all', '--json']);
  assert.strictEqual(added.status, 0, `skills add failed:\n${added.stdout}\n${added.stderr}`);
  const entries = jsonOf(added);
  assert.deepStrictEqual(
    entries.map((entry) => [entry.name, entry.status]),
    [['jag', 'installed']],
  );

  for (const [folder, ids] of folders) {
    assert.ok(
      fs.existsSync(path.join(project, folder, 'jag', 'SKILL.md')),
      `the skill is not in ${folder} (${ids.join(', ')})`,
    );
    const file = path.join(project, folder, 'jag', 'SKILL.md');
    if (ids.includes('eve')) {
      sameSkill(file);
    } else {
      sameBytes(file);
    }
  }
  // Claude Code, by name.
  sameBytes(path.join(project, '.claude', 'skills', 'jag', 'SKILL.md'));

  const listed = jsonOf(skills(project, ['list', '--json']));
  assert.deepStrictEqual(
    listed.map((entry) => entry.name),
    ['jag'],
  );
  assert.ok(listed[0].agents.includes('Claude Code'));

  // The README says that the tool writes `skills-lock.json` in the project.
  const lock = JSON.parse(fs.readFileSync(path.join(project, 'skills-lock.json'), 'utf8'));
  assert.deepStrictEqual(Object.keys(lock.skills), ['jag']);
});

// The README says that `--agent claude-code -y` asks no question.
test('the install for Claude Code with -y asks no question', () => {
  const project = temporaryFolder('claude');
  const added = skills(project, ['add', checkout, '--agent', 'claude-code', '-y']);
  assert.strictEqual(added.status, 0, `skills add failed:\n${added.stdout}\n${added.stderr}`);
  sameBytes(path.join(project, '.claude', 'skills', 'jag', 'SKILL.md'));
});
