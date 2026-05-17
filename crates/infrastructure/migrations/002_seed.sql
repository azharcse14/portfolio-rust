-- Seed default content so the site looks identical to the legacy template
-- on first run. Edit later via the admin dashboard (Phase 4).

INSERT INTO profile (id, name, title, bio, photo_url, email, github, linkedin, twitter)
VALUES (
    1,
    'Azharul Islam',
    'Full Stack Developer',
    'Hi, I''m Azharul Islam — a full stack developer who enjoys building well-architected, fast products. I work across the stack with Rust, TypeScript, and the boring-but-important parts of software design. Currently focused on Rust + Leptos for the web, with a soft spot for clean architecture, fast feedback loops, and software that holds up under maintenance.',
    '/images/about.jpg',
    'mdazharcse14@gmail.com',
    'https://github.com/azharcse14',
    NULL,
    NULL
);

-- 6 skill progress bars matching the legacy template
INSERT INTO skills (id, name, icon, category, percentage, color, display_order) VALUES
    ('11111111-0000-0000-0000-000000000001', 'Rust',            NULL, 'Language', 85, 'color-1', 1),
    ('11111111-0000-0000-0000-000000000002', 'TypeScript',      NULL, 'Language', 90, 'color-2', 2),
    ('11111111-0000-0000-0000-000000000003', 'Leptos / React',  NULL, 'Frontend', 80, 'color-3', 3),
    ('11111111-0000-0000-0000-000000000004', 'TailwindCSS',     NULL, 'Frontend', 90, 'color-4', 4),
    ('11111111-0000-0000-0000-000000000005', 'Postgres / SQL',  NULL, 'Data',     75, 'color-5', 5),
    ('11111111-0000-0000-0000-000000000006', 'Docker / DevOps', NULL, 'DevOps',   70, 'color-6', 6);

-- 6 projects matching the legacy Work section
INSERT INTO projects (id, title, slug, description, image_url, tech_stack, github_url, live_url, featured, display_order, created_at) VALUES
    ('22222222-0000-0000-0000-000000000001', 'Portfolio (Rust + Leptos)', 'portfolio-rust',
        'This very site — Cargo workspace with 4 crates following clean architecture: domain / application / infrastructure / presentation. Leptos 0.7 SSR + Axum, SQLite-backed via SQLx.',
        '/images/img-1.jpg',
        '["Rust","Leptos","Axum","TailwindCSS","SQLx","SQLite"]',
        'https://github.com/azharcse14/portfolio-rust', NULL, 1, 1, datetime('now')),

    ('22222222-0000-0000-0000-000000000002', 'Task Tracker', 'task-tracker',
        'Minimalist kanban-style task manager with offline-first sync and keyboard shortcuts.',
        '/images/img-2.jpg',
        '["TypeScript","React","IndexedDB","Vite"]',
        'https://github.com/azharcse14/task-tracker', NULL, 1, 2, datetime('now')),

    ('22222222-0000-0000-0000-000000000003', 'Weather CLI', 'weather-cli',
        'Beautiful terminal weather forecast using Open-Meteo, rendered with Ratatui — charts, sunrise/sunset, 7-day outlook.',
        '/images/img-3.jpg',
        '["Rust","Ratatui","Tokio","reqwest"]',
        'https://github.com/azharcse14/weather-cli', NULL, 1, 3, datetime('now')),

    ('22222222-0000-0000-0000-000000000004', 'Image Optimizer', 'image-optimizer',
        'Bulk image compression service — drop a folder, get optimized WebP/AVIF. Supports CLI + web upload.',
        '/images/img-4.jpg',
        '["Rust","Axum","image-rs","WebAssembly"]',
        'https://github.com/azharcse14/image-optimizer', NULL, 0, 4, datetime('now')),

    ('22222222-0000-0000-0000-000000000005', 'Markdown Presenter', 'markdown-presenter',
        'Turn .md files into beautiful slide decks. Themes, syntax highlighting, presenter notes.',
        '/images/img-5.jpg',
        '["Svelte","Vite","Marked","highlight.js"]',
        'https://github.com/azharcse14/markdown-presenter', NULL, 0, 5, datetime('now')),

    ('22222222-0000-0000-0000-000000000006', 'URL Shortener', 'url-shortener',
        'Fast self-hosted link shortener with analytics dashboard, click tracking, and custom slugs.',
        '/images/img-6.jpg',
        '["Rust","Actix-web","SQLite","HTMX"]',
        'https://github.com/azharcse14/url-shortener', NULL, 0, 6, datetime('now'));

-- 3 blog posts matching the legacy Blog section
INSERT INTO posts (id, title, slug, content_md, cover_image, tags, published, published_at, created_at) VALUES
    ('33333333-0000-0000-0000-000000000001', 'Why I chose Rust + Leptos for my portfolio', 'why-rust-for-the-web',
        '# Why Rust + Leptos

Most portfolios use Next.js or plain HTML. So why Rust?

## Speed of iteration

Surprisingly, **Leptos** is fast to build with once you internalize signals. The compile times hurt, but the developer experience after the first build is fluid.

## Single language

From HTTP handlers down to UI components — one language, one mental model.

## Tradeoffs

- **Compile times** — workspace setup helps but isn''t free.
- **Ecosystem maturity** — fewer components, more building from scratch.
- **WASM payload** — bigger than React for cold loads.

For a portfolio, the tradeoffs are worth it: a fast site, a great learning experience, and code you actually enjoy reading later.',
        '/images/blog-1.jpg',
        '["Rust","Leptos","Architecture"]',
        1, datetime('now'), datetime('now')),

    ('33333333-0000-0000-0000-000000000002', 'Clean architecture in Rust with a Cargo workspace', 'clean-architecture-rust',
        '# Clean architecture in Rust

The dependency rule is simple: **outer layers depend on inner ones, never the reverse.**

## The four crates

1. `domain` — entities, value objects, repository **traits**. Pure Rust, zero I/O.
2. `application` — use-cases that orchestrate domain via traits.
3. `infrastructure` — concrete adapters (SQLx, Resend, JWT) implementing those traits.
4. `presentation` — Leptos UI + Axum, wires everything together in a composition root.

## Why it pays off

- **Testability** — domain + application tested without spinning up a DB.
- **Replaceable infra** — swap Postgres for SQLite by adding a new adapter, not by editing business logic.
- **Clarity** — when someone reads a use-case, there''s no SQL or HTTP noise in the way.',
        '/images/blog-2.jpg',
        '["Architecture","Rust","Patterns"]',
        1, datetime('now'), datetime('now')),

    ('33333333-0000-0000-0000-000000000003', 'Three small TailwindCSS v4 tips', 'tailwind-v4-tips',
        '# TailwindCSS v4: three small wins

## 1. Zero-config

Delete `tailwind.config.js`. Use `@import "tailwindcss"` and customize with `@theme {}` in your CSS.

## 2. Native CSS variables for theme

Everything you define under `@theme` becomes a CSS variable — accessible to your own CSS *and* to Tailwind utilities.

## 3. Faster builds

The Rust-based engine is noticeably quicker. Combined with cargo-leptos''s built-in Tailwind binary, dev reload feels instant.',
        '/images/blog-3.jpg',
        '["TailwindCSS","Frontend","Tips"]',
        1, datetime('now'), datetime('now'));
