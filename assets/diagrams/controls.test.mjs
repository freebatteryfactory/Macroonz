import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile, writeFile, mkdir, mkdtemp, rm, copyFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { catalog, compare, outputs, execute, here, repository } from './run.mjs';
import { staged } from './staged.mjs';

const scratch = path.join(repository, 'target/qualification/documentation-navigation');
process.env.PUPPETEER_CACHE_DIR = path.join(scratch, 'browser');
const git = (root, ...args) => execFileSync('git', ['-C', root, ...args], { stdio: 'pipe' });
const read = file => readFile(file, 'utf8');

async function fixture() {
  await mkdir(scratch, { recursive: true });
  const root = await mkdtemp(path.join(scratch, 'control-'));
  await mkdir(path.join(root, 'assets/diagrams'), { recursive: true });
  for (const name of ['run.mjs', 'render.json', 'staged.mjs', 'package.json', 'bun.lock', 'OFL.txt', 'facade-1.mmd', 'facade-1.svg']) {
    await copyFile(path.join(here, name), path.join(root, 'assets/diagrams', name));
  }
  await writeFile(path.join(root, 'assets/diagrams/catalog.json'), JSON.stringify([
    { id: 'facade-1', package: '.', readme: 'README.md', rustdoc: false },
  ]));
  await writeFile(path.join(root, 'README.md'), '![Packages][diagram-facade-1]\n\n[Diagram source](assets/diagrams/facade-1.mmd)\n\n[diagram-facade-1]: assets/diagrams/facade-1.svg\n');
  return root;
}

test('missing and orphaned authored sources refuse before rendering', async () => {
  const root = await fixture();
  try {
    const source = path.join(root, 'assets/diagrams/facade-1.mmd');
    await rm(source);
    await assert.rejects(catalog(root), /Missing or orphaned/);
    await copyFile(path.join(here, 'facade-1.mmd'), source);
    await writeFile(path.join(root, 'assets/diagrams/foreign.mmd'), 'graph LR\n');
    await assert.rejects(catalog(root), /Missing or orphaned/);
  } finally { await rm(root, { recursive: true, force: true }); }
});

test('comparison distinguishes missing, stale and orphaned images without repairing them', async () => {
  const root = await fixture();
  try {
    const expected = new Map([['assets/diagrams/facade-1.svg', '<svg>expected</svg>\n']]);
    const image = path.join(root, 'assets/diagrams/facade-1.svg');
    await writeFile(image, '<svg>stale</svg>\n');
    assert.deepEqual(await compare(root, expected), ['Stale: assets/diagrams/facade-1.svg']);
    assert.equal(await read(image), '<svg>stale</svg>\n');
    await rm(image);
    await writeFile(path.join(root, 'assets/diagrams/foreign.svg'), '<svg/>');
    assert.deepEqual(await compare(root, expected), ['Missing: assets/diagrams/facade-1.svg', 'Orphan: assets/diagrams/foreign.svg']);
  } finally { await rm(root, { recursive: true, force: true }); }
});

test('render failure leaves the existing batch unchanged', async () => {
  const root = await fixture();
  const original = await read(path.join(root, 'assets/diagrams/facade-1.svg'));
  try {
    await assert.rejects(outputs(root, async () => { throw new Error('failed renderer'); }), /failed renderer/);
    assert.equal(await read(path.join(root, 'assets/diagrams/facade-1.svg')), original);
    const rows = await catalog(root);
    rows.push({ id: 'broken-second', package: '.', readme: 'README.md', rustdoc: false });
    await writeFile(path.join(root, 'assets/diagrams/catalog.json'), JSON.stringify(rows));
    const firstSource = path.join(root, 'assets/diagrams/facade-1.mmd');
    await writeFile(firstSource, (await read(firstSource)).replace('your crate', 'changed adopter'));
    await writeFile(path.join(root, 'assets/diagrams/broken-second.mmd'), 'flowchart LR\n accTitle: Invalid\n accDescr: Literal invalid syntax\n A -->[\n');
    await writeFile(path.join(root, 'assets/diagrams/broken-second.svg'), '<svg>previous second image</svg>\n');
    process.env.MACROONZ_DIAGRAM_TOOLS = here;
    await assert.rejects(execute('generate', root));
    assert.equal(await read(path.join(root, 'assets/diagrams/facade-1.svg')), original);
    assert.equal(await read(path.join(root, 'assets/diagrams/broken-second.svg')), '<svg>previous second image</svg>\n');
  } finally { delete process.env.MACROONZ_DIAGRAM_TOOLS; await rm(root, { recursive: true, force: true }); }
});

test('staged snapshot ignores unstaged repairs and preserves index and working bytes', async () => {
  const root = await fixture();
  try {
    git(root, 'init', '--quiet');
    git(root, 'add', '.');
    const source = path.join(root, 'assets/diagrams/facade-1.mmd');
    const original = await read(source);
    // Valid staged diagram must pass even with an invalid working-tree copy.
    await writeFile(source, 'unparseable working tree');
    await staged(root, here);
    assert.equal(await read(source), 'unparseable working tree');
    // The opposite arrangement must refuse, retaining both versions exactly.
    const changed = original.replace('your crate', 'changed adopter');
    await writeFile(source, changed);
    git(root, 'add', 'assets/diagrams/facade-1.mmd');
    await writeFile(source, original);
    const index = git(root, 'ls-files', '-s').toString();
    await assert.rejects(staged(root, here), /Stale/);
    assert.equal(git(root, 'ls-files', '-s').toString(), index);
    assert.equal(await read(source), original);
    git(root, 'add', 'assets/diagrams/facade-1.mmd');
    const configPath = path.join(root, 'assets/diagrams/render.json');
    const config = await read(configPath);
    await writeFile(configPath, config.replace('"default"', '"forest"'));
    git(root, 'add', 'assets/diagrams/render.json');
    await writeFile(configPath, config);
    await assert.rejects(staged(root, here), /Stale/);
    git(root, 'add', 'assets/diagrams/render.json');
    const driver = path.join(root, 'assets/diagrams/run.mjs');
    const goodDriver = await read(driver);
    await writeFile(driver, 'throw new Error("literal staged driver failure");\n');
    git(root, 'add', 'assets/diagrams/run.mjs');
    await writeFile(driver, goodDriver);
    await assert.rejects(staged(root, here), /literal staged driver failure/);
    git(root, 'add', 'assets/diagrams/run.mjs');
    await assert.rejects(staged(root, path.join(root, 'absent-tools')), /ENOENT/);
  } finally { await rm(root, { recursive: true, force: true }); }
});
