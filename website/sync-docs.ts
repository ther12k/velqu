/**
 * Deterministic docs sync for the Starlight site.
 *
 * `docs/` remains the canonical home of documentation; this script copies
 * curated pages into the Starlight content directory with site frontmatter
 * and rewritten links, so the site can never drift from the repository.
 * Runs before every build (`bun run build`) and in CI.
 */
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "..");
const OUT = join(import.meta.dir, "src", "content", "docs");
const SITE_BASE = "/velqu";
const REPO_BLOB = "https://github.com/ther12k/velqu/blob/master";
const REPO_TREE = "https://github.com/ther12k/velqu/tree/master";

interface PageSpec {
  /** Repo-relative source path. */
  source: string;
  slug: string;
  title: string;
  description: string;
}

const PAGES: PageSpec[] = [
  {
    source: "docs/beta/BROWSER_WASM.md",
    slug: "browser-wasm",
    title: "Browser-WASM",
    description:
      "Build a Velqu app to static assets that run in an ordinary browser — Rust/WASM kernel, isolated Worker handlers, no Velqu application server.",
  },
  {
    source: "docs/beta/INSTALL.md",
    slug: "deployment",
    title: "Deployment (shared mode)",
    description:
      "Deploy the Rust runtime binary plus a verified QPack; update policy and enforced limits.",
  },
  {
    source: "docs/beta/KNOWN-LIMITATIONS.md",
    slug: "known-limitations",
    title: "Known limitations",
    description:
      "Recorded limitations and the honesty policy: trusted code only, no parity claims without evidence.",
  },
];

const SITE_SLUGS = new Map(PAGES.map((page) => [page.source, page.slug]));

/** Strip a leading YAML frontmatter block, if present. */
function stripFrontmatter(content: string): string {
  if (!content.startsWith("---\n")) return content;
  const close = content.indexOf("\n---\n", 4);
  if (close === -1) return content;
  return content.slice(close + 5);
}

/** Drop the source doc's leading H1 — Starlight renders the frontmatter title itself. */
function stripLeadingH1(body: string): string {
  return body.replace(/^\s*# [^\n]*\n+/, "");
}

/** Resolve a Markdown link target relative to the source doc's repo directory. */
function resolveRepoPath(sourceDoc: string, target: string): string {
  const baseDir = dirname(join("/", sourceDoc));
  return normalize(join(baseDir, target)).replace(/^\//, "").replace(/\\/g, "/");
}

/** Rewrite repo-relative links: synced pages become site routes, everything else points at GitHub. */
function rewriteLinks(body: string, sourceDoc: string): string {
  return body.replace(/\]\(([^)\s]+)(?:\s+"[^"]*")?\)/g, (full, rawTarget: string) => {
    const target = rawTarget.trim();
    if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(target) || target.startsWith("#") || target.startsWith("/")) {
      return full;
    }
    const [pathPart, ...fragments] = target.split("#");
    const fragment = fragments.length > 0 ? `#${fragments.join("#")}` : "";
    if (pathPart === undefined || pathPart === "") return full;
    const resolved = resolveRepoPath(sourceDoc, pathPart);
    const slug = SITE_SLUGS.get(resolved);
    if (slug !== undefined) return `](${SITE_BASE}/${slug}/${fragment})`;
    if (resolved.endsWith(".md")) return `](${REPO_BLOB}/${resolved}${fragment})`;
    return `](${REPO_TREE}/${resolved})`;
  });
}

// Remove only this script's own outputs so committed pages (index.mdx,
// getting-started.md) survive.
mkdirSync(OUT, { recursive: true });
for (const page of PAGES) {
  rmSync(join(OUT, `${page.slug}.md`), { force: true });
}

for (const page of PAGES) {
  const source = join(ROOT, page.source);
  const body = stripLeadingH1(
    rewriteLinks(stripFrontmatter(readFileSync(source, "utf8")).trimEnd(), page.source),
  ).trimEnd();
  const frontmatter = [
    "---",
    `title: ${JSON.stringify(page.title)}`,
    `description: ${JSON.stringify(page.description)}`,
    "---",
    "",
  ].join("\n");
  writeFileSync(join(OUT, `${page.slug}.md`), `${frontmatter}${body}\n`);
}
