import { readFile, writeFile, mkdir, readdir, mkdtemp, rm, lstat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';

export const here = path.dirname(fileURLToPath(import.meta.url));
export const repository = path.resolve(here, '../..');
const slash = value => value.split(path.sep).join('/');
const text = file => readFile(file, 'utf8');
const json = async file => JSON.parse(await text(file));

export function within(root, name) {
  if (!name || path.isAbsolute(name) || name.includes('\\') || name.split('/').includes('..')) {
    throw new Error(`Unconfined path: ${name}`);
  }
  return path.join(root, name);
}

export async function catalog(root) {
  const rows = await json(path.join(root, 'assets/diagrams/catalog.json'));
  if (!Array.isArray(rows) || !rows.length) throw new Error('Empty diagram catalog');
  const ids = new Set();
  for (const row of rows) {
    if (!/^[a-z0-9-]+$/.test(row.id) || ids.has(row.id)) throw new Error(`Invalid/repeated diagram: ${row.id}`);
    ids.add(row.id);
    within(root, row.readme);
    if (!['.', 'harness', 'macros/compiler'].includes(row.package)) throw new Error('Unknown delivery package');
    if (row.package !== '.' && !row.readme.startsWith(`${row.package}/`)) throw new Error('Foreign delivery');
    if (typeof row.rustdoc !== 'boolean') throw new Error('Missing rustdoc posture');
  }
  const sources = (await readdir(path.join(root, 'assets/diagrams'))).filter(name => name.endsWith('.mmd'));
  if (sources.length !== ids.size || sources.some(name => !ids.has(name.slice(0, -4)))) {
    throw new Error('Missing or orphaned authored Mermaid source');
  }
  return rows;
}

export function imagePath(row) {
  return `${row.package === '.' ? '' : row.package + '/'}assets/diagrams/${row.id}.svg`;
}

export async function outputs(root, render) {
  const rows = await catalog(root);
  const result = new Map();
  const definitions = new Map();
  for (const row of rows) {
    const source = await text(path.join(root, `assets/diagrams/${row.id}.mmd`));
    if (!source.match(/^\s*accTitle: .+$/m) || !source.match(/^\s*accDescr: .+$/m)) {
      throw new Error(`Missing accessibility description: ${row.id}`);
    }
    const svg = await render(source, row.id);
    if (!svg.includes('<svg') || !svg.includes('</svg>')) throw new Error(`Renderer produced no SVG: ${row.id}`);
    result.set(`assets/diagrams/${row.id}.svg`, svg);
    if (row.package !== '.') {
      result.set(imagePath(row), svg);
      result.set(imagePath(row).replace(/\.svg$/, '.mmd'), source);
    }
    if (row.rustdoc) {
      const file = slash(path.join(path.dirname(row.readme), 'diagrams.md'));
      const old = definitions.get(file) ?? '';
      definitions.set(file, `${old}[diagram-${row.id}]: data:image/svg+xml;base64,${Buffer.from(svg).toString('base64')}\n`);
    }
  }
  for (const [file, value] of definitions) result.set(file, value);
  const license = await text(path.join(root, 'assets/diagrams/OFL.txt'));
  for (const owner of new Set(rows.map(row => row.package))) {
    if (owner !== '.') result.set(`${owner}/assets/diagrams/OFL.txt`, license);
  }
  return { rows, files: result };
}

export async function compare(root, files) {
  const problems = [];
  for (const [name, expected] of files) {
    try {
      if (await text(within(root, name)) !== expected) problems.push(`Stale: ${name}`);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      problems.push(`Missing: ${name}`);
    }
  }
  for (const directory of ['assets/diagrams', 'harness/assets/diagrams', 'macros/compiler/assets/diagrams']) {
    for (const name of await readdir(path.join(root, directory)).catch(error => {
      if (error.code === 'ENOENT') return [];
      throw error;
    })) {
      const file = `${directory}/${name}`;
      if ((name.endsWith('.svg') || (directory !== 'assets/diagrams' && name.endsWith('.mmd'))) && !files.has(file)) {
        problems.push(`Orphan: ${file}`);
      }
    }
  }
  for (const owner of ['harness/src', 'macros/compiler/src']) {
    const pending = [owner];
    while (pending.length) {
      const directory = pending.pop();
      const entries = await readdir(path.join(root, directory), { withFileTypes: true }).catch(error => {
        if (error.code === 'ENOENT') return [];
        throw error;
      });
      for (const entry of entries) {
        const name = `${directory}/${entry.name}`;
        if (entry.isDirectory()) pending.push(name);
        else if (entry.name === 'diagrams.md' && !files.has(name)) problems.push(`Orphan: ${name}`);
      }
    }
  }
  return problems;
}

export async function delivery(root, rows) {
  for (const row of rows) {
    const readme = await text(within(root, row.readme));
    const relative = slash(path.relative(path.dirname(row.readme), imagePath(row)));
    if (!readme.includes(`[diagram-${row.id}]: ${relative}\n`) || !readme.includes(`][diagram-${row.id}]`)) {
      throw new Error(`Missing README image reference: ${row.readme} / ${row.id}`);
    }
    const sourcePointer = row.rustdoc
      ? `Diagram source: \`assets/diagrams/${row.id}.mmd\` in this crate's source package.`
      : `(${relative.replace(/\.svg$/, '.mmd')})`;
    if (!readme.includes(sourcePointer)) throw new Error(`Missing source pointer: ${row.id}`);
    if (readme.includes('```mermaid')) throw new Error(`Unconverted Mermaid block: ${row.readme}`);
    if (row.rustdoc) {
      const module = await text(path.join(root, path.dirname(row.readme), 'mod.rs'));
      if (!module.includes('include_str!("diagrams.md")') || !module.includes('include_str!("README.md")')) {
        throw new Error(`Missing offline rustdoc delivery: ${row.readme}`);
      }
    }
  }
}

export async function renderer(root) {
  const tools = process.env.MACROONZ_DIAGRAM_TOOLS ?? here;
  if (await text(path.join(root, 'assets/diagrams/bun.lock')) !== await text(path.join(tools, 'bun.lock'))) {
    throw new Error('Staged dependency lock differs; install its locked toolchain before checking');
  }
  const manifest = await json(path.join(root, 'assets/diagrams/package.json'));
  if (process.versions.node !== manifest.engines.node) throw new Error(`Use Node ${manifest.engines.node}, got ${process.versions.node}`);
  for (const [name, version] of Object.entries(manifest.dependencies)) {
    const actual = await json(path.join(tools, 'node_modules', name, 'package.json'));
    if (actual.version !== version) throw new Error(`Install the locked renderer: ${name}`);
  }
  const resolve = createRequire(path.join(tools, 'package.json')).resolve;
  const { default: puppeteer } = await import(pathToFileURL(resolve('puppeteer')));
  const { renderMermaid } = await import(pathToFileURL(resolve('@mermaid-js/mermaid-cli')));
  const config = await json(path.join(root, 'assets/diagrams/render.json'));
  const font = await readFile(path.join(tools, 'node_modules/@fontsource/noto-sans/files/noto-sans-latin-400-normal.woff2'));
  const css = `@font-face{font-family:'Noto Sans';font-style:normal;font-weight:400;src:url(data:font/woff2;base64,${font.toString('base64')}) format('woff2');}`;
  const scratch = path.join(repository, 'target/qualification/documentation-navigation');
  await mkdir(scratch, { recursive: true });
  const profile = await mkdtemp(path.join(scratch, 'browser-profile-'));
  let browser;
  try {
    browser = await puppeteer.launch({ headless: 'shell', userDataDir: profile });
    if (!(await browser.version()).endsWith(`/${config.browser}`)) throw new Error('Unexpected browser version');
    // The CLI uses only newPage on its Browser/BrowserContext argument.
    // Prepare the page before its renderer starts so layout and the emitted image use the same embedded font.
    const pages = { async newPage() {
      const page = await browser.newPage();
      await page.setOfflineMode(true);
      await page.evaluateOnNewDocument(style => {
        document.addEventListener('DOMContentLoaded', () => {
          const node = document.createElement('style');
          node.textContent = style;
          document.head.append(node);
        }, { once: true });
      }, css);
      return page;
    } };
    return {
      async render(source, id) {
        const { data } = await renderMermaid(pages, source, 'svg', {
          mermaidConfig: config.mermaid, viewport: config.viewport,
          backgroundColor: 'white', myCSS: css, svgId: `diagram-${id}`,
        });
        return Buffer.from(data).toString('utf8') + '\n';
      },
      async close() { await browser.close(); await rm(profile, { recursive: true, force: true }); },
    };
  } catch (error) {
    await browser?.close();
    await rm(profile, { recursive: true, force: true });
    throw error;
  }
}

export async function execute(mode, root = repository) {
  if (!['generate', 'check'].includes(mode)) throw new Error('Use generate or check');
  const engine = await renderer(root);
  try {
    // Finish the whole render before writing any delivery; renderer failure never installs a partial batch.
    const { rows, files } = await outputs(root, engine.render);
    await delivery(root, rows);
    const problems = await compare(root, files);
    if (mode === 'check' && problems.length) throw new Error(problems.join('\n'));
    if (mode === 'generate') {
      if (problems.some(problem => problem.startsWith('Orphan:'))) throw new Error(problems.join('\n'));
      for (const [name, value] of files) {
        const file = within(root, name);
        await mkdir(path.dirname(file), { recursive: true });
        try { if ((await lstat(file)).isSymbolicLink()) throw new Error(`Linked output: ${name}`); }
        catch (error) { if (error.code !== 'ENOENT') throw error; }
        await writeFile(file, value);
      }
    }
    console.log(`${mode}: ${rows.length} diagrams; ${files.size} generated deliveries; ${process.platform}/${process.arch}; Node ${process.versions.node}`);
  } finally { await engine.close(); }
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  process.env.PUPPETEER_CACHE_DIR ??= path.join(repository, 'target/qualification/documentation-navigation/browser');
  try { await execute(process.argv[2]); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
