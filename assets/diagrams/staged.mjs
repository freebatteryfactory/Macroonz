import { execFileSync, spawnSync } from 'node:child_process';
import { mkdir, mkdtemp, writeFile, rm } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const git = (root, ...args) => execFileSync('git', ['-C', root, ...args], { maxBuffer: 16 * 1024 * 1024 });

export async function staged(root, tools = process.env.MACROONZ_DIAGRAM_TOOLS ?? here) {
  const changed = git(root, 'diff', '--cached', '--name-only', '-z').toString().split('\0').filter(Boolean);
  const catalog = JSON.parse(git(root, 'show', ':assets/diagrams/catalog.json').toString());
  const docs = new Set(catalog.flatMap(row => [row.readme, path.posix.join(path.posix.dirname(row.readme), 'diagrams.md'), path.posix.join(path.posix.dirname(row.readme), 'mod.rs')]));
  if (!changed.some(name => docs.has(name) || /(^|\/)(assets\/diagrams\/|diagrams\.md$)|^\.githooks\/|^\.github\/workflows\/documentation\.yml$/.test(name))) {
    console.log('staged: no diagram inputs or delivery documents changed');
    return;
  }
  const scratch = path.join(root, 'target/qualification/documentation-navigation');
  await mkdir(scratch, { recursive: true });
  const snapshot = await mkdtemp(path.join(scratch, 'index-'));
  try {
    const index = git(root, 'ls-files', '-s', '-z').toString().split('\0').filter(Boolean);
    for (const entry of index) {
      const [mode, hash, stage, name] = entry.split(/[ \t]/, 4);
      if (!(docs.has(name) || name.endsWith('/diagrams.md') || /^(assets\/diagrams|harness\/assets\/diagrams|macros\/compiler\/assets\/diagrams)\//.test(name))) continue;
      if (stage !== '0' || !['100644', '100755'].includes(mode) || name.split('/').includes('..')) throw new Error(`Unresolved/linked input: ${name}`);
      const file = path.join(snapshot, name);
      await mkdir(path.dirname(file), { recursive: true });
      await writeFile(file, git(root, 'cat-file', 'blob', hash));
    }
    const result = spawnSync(process.execPath, [path.join(snapshot, 'assets/diagrams/run.mjs'), 'check'], {
      cwd: snapshot, stdio: 'pipe', encoding: 'utf8', timeout: 180000,
      env: { ...process.env, MACROONZ_DIAGRAM_TOOLS: tools,
        PUPPETEER_CACHE_DIR: process.env.PUPPETEER_CACHE_DIR ?? path.join(scratch, 'browser') },
    });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(result.stderr || result.stdout || `Staged renderer exited ${result.status}`);
    console.log(result.stdout.trim());
  } finally { await rm(snapshot, { recursive: true, force: true }); }
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  try { await staged(git(process.cwd(), 'rev-parse', '--show-toplevel').toString().trim()); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
