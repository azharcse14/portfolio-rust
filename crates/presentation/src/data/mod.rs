//! Hardcoded sample content used by Phase 2 UI.
//! Will be replaced by DB-backed server functions in Phase 3.

#[derive(Debug, Clone)]
pub struct SampleProject {
    pub slug: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub github: Option<&'static str>,
    pub live: Option<&'static str>,
    pub featured: bool,
    pub emoji: &'static str,
    pub gradient: &'static str,
}

#[derive(Debug, Clone)]
pub struct SamplePost {
    pub slug: &'static str,
    pub title: &'static str,
    pub excerpt: &'static str,
    pub body_md: &'static str,
    pub date: &'static str,
    pub tags: &'static [&'static str],
    pub read_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct SampleSkill {
    pub name: &'static str,
    pub category: &'static str,
}

#[derive(Debug, Clone)]
pub struct SampleExperience {
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub description: &'static str,
}

pub const PROJECTS: &[SampleProject] = &[
    SampleProject {
        slug: "portfolio-rust",
        title: "Portfolio (Rust + Leptos)",
        tagline: "Fullstack portfolio with admin dashboard, built with clean architecture.",
        description: "This very site — Cargo workspace with 4 crates (domain / application / infrastructure / presentation), Leptos 0.7 SSR, Axum, TailwindCSS, and a Postgres-backed admin panel.",
        tech: &["Rust", "Leptos", "Axum", "TailwindCSS", "SQLx", "Postgres"],
        github: Some("https://github.com/azharcse14/portfolio-rust"),
        live: None,
        featured: true,
        emoji: "🦀",
        gradient: "from-orange-500 to-pink-500",
    },
    SampleProject {
        slug: "task-tracker",
        title: "Task Tracker",
        tagline: "Minimalist kanban-style task manager.",
        description: "A focused todo + kanban app with offline-first sync, keyboard shortcuts, and zero distractions.",
        tech: &["TypeScript", "React", "IndexedDB", "Vite"],
        github: Some("https://github.com/azharcse14/task-tracker"),
        live: None,
        featured: true,
        emoji: "✅",
        gradient: "from-emerald-500 to-teal-500",
    },
    SampleProject {
        slug: "weather-cli",
        title: "Weather CLI",
        tagline: "Beautiful terminal weather forecast.",
        description: "Fetches forecasts from Open-Meteo and renders them in a rich TUI with charts, sunrise/sunset, and 7-day outlook.",
        tech: &["Rust", "Ratatui", "Tokio", "reqwest"],
        github: Some("https://github.com/azharcse14/weather-cli"),
        live: None,
        featured: true,
        emoji: "🌤️",
        gradient: "from-sky-500 to-indigo-500",
    },
    SampleProject {
        slug: "image-optimizer",
        title: "Image Optimizer",
        tagline: "Bulk image compression service.",
        description: "Drop a folder of images, get optimized WebP/AVIF outputs. Supports CLI + web upload.",
        tech: &["Rust", "Axum", "image-rs", "WebAssembly"],
        github: Some("https://github.com/azharcse14/image-optimizer"),
        live: None,
        featured: false,
        emoji: "🖼️",
        gradient: "from-fuchsia-500 to-purple-500",
    },
    SampleProject {
        slug: "markdown-presenter",
        title: "Markdown Presenter",
        tagline: "Turn .md files into beautiful slide decks.",
        description: "Browser-based presentation tool — write Markdown, get instant slides with themes, syntax highlighting, and presenter notes.",
        tech: &["Svelte", "Vite", "Marked", "highlight.js"],
        github: Some("https://github.com/azharcse14/markdown-presenter"),
        live: None,
        featured: false,
        emoji: "📊",
        gradient: "from-rose-500 to-orange-500",
    },
    SampleProject {
        slug: "url-shortener",
        title: "URL Shortener",
        tagline: "Fast, self-hosted link shortener.",
        description: "Single-binary URL shortener with analytics dashboard, click tracking, and custom slugs. SQLite by default.",
        tech: &["Rust", "Actix-web", "SQLite", "HTMX"],
        github: Some("https://github.com/azharcse14/url-shortener"),
        live: None,
        featured: false,
        emoji: "🔗",
        gradient: "from-cyan-500 to-blue-500",
    },
];

pub const POSTS: &[SamplePost] = &[
    SamplePost {
        slug: "why-rust-for-the-web",
        title: "Why I chose Rust + Leptos for my portfolio",
        excerpt: "A pragmatic look at the tradeoffs of using a young framework for a personal site.",
        body_md: "# Why Rust + Leptos\n\nMost portfolios use Next.js or plain HTML. So why Rust?\n\n## Speed of iteration\n\nSurprisingly, **Leptos** is fast to build with once you internalize signals. The compile times hurt, but the developer experience after the first build is fluid.\n\n## Single language\n\nFrom HTTP handlers down to UI components — one language, one mental model.\n\n```rust\n#[server]\npub async fn list_projects() -> Result<Vec<Project>, ServerFnError> {\n    // runs on the server, called like a normal fn from the client\n    Ok(repo().list(false).await?)\n}\n```\n\n## Tradeoffs\n\n- **Compile times** — workspace setup helps but isn't free.\n- **Ecosystem maturity** — fewer components, more building from scratch.\n- **WASM payload** — bigger than React for cold loads.\n\nFor a portfolio, the tradeoffs are worth it: you get a fast site, a great learning experience, and code you actually enjoy reading later.",
        date: "2026-05-15",
        tags: &["Rust", "Leptos", "Architecture"],
        read_minutes: 5,
    },
    SamplePost {
        slug: "clean-architecture-rust",
        title: "Clean architecture in Rust with a Cargo workspace",
        excerpt: "Splitting a project into domain / application / infrastructure / presentation crates.",
        body_md: "# Clean architecture in Rust\n\nThe dependency rule is simple: **outer layers depend on inner ones, never the reverse.**\n\n## The four crates\n\n1. `domain` — entities, value objects, repository **traits**. Pure Rust, zero I/O.\n2. `application` — use-cases that orchestrate domain via traits.\n3. `infrastructure` — concrete adapters (SQLx, Resend, JWT) implementing those traits.\n4. `presentation` — Leptos UI + Axum, wires everything together in a composition root.\n\n## Why it pays off\n\n- **Testability** — domain + application tested without spinning up a DB.\n- **Replaceable infra** — swap Postgres for SQLite by adding a new adapter, not by editing business logic.\n- **Clarity** — when someone reads a use-case, there's no SQL or HTTP noise in the way.\n\n## The cost\n\nMore files. More boilerplate at first. Worth it once the project grows past a single binary.",
        date: "2026-05-10",
        tags: &["Architecture", "Rust", "Patterns"],
        read_minutes: 7,
    },
    SamplePost {
        slug: "tailwind-v4-tips",
        title: "Three small TailwindCSS v4 tips",
        excerpt: "What changed in v4 and the conventions I now follow.",
        body_md: "# TailwindCSS v4: three small wins\n\nThe migration from v3 to v4 mostly *just works*, but here are the bits that improved my workflow.\n\n## 1. Zero-config\n\nDelete `tailwind.config.js`. Use `@import \"tailwindcss\"` and customize with `@theme {}` in your CSS.\n\n## 2. Native CSS variables for theme\n\nEverything you define under `@theme` becomes a CSS variable — accessible to your own CSS *and* to Tailwind utilities.\n\n## 3. Faster builds\n\nThe Rust-based engine is noticeably quicker. Combined with `cargo-leptos`'s built-in Tailwind binary, dev reload feels instant.",
        date: "2026-04-28",
        tags: &["TailwindCSS", "Frontend", "Tips"],
        read_minutes: 3,
    },
];

pub const SKILLS: &[SampleSkill] = &[
    SampleSkill { name: "Rust", category: "Language" },
    SampleSkill { name: "TypeScript", category: "Language" },
    SampleSkill { name: "Python", category: "Language" },
    SampleSkill { name: "SQL", category: "Language" },
    SampleSkill { name: "Leptos", category: "Frontend" },
    SampleSkill { name: "React", category: "Frontend" },
    SampleSkill { name: "TailwindCSS", category: "Frontend" },
    SampleSkill { name: "Svelte", category: "Frontend" },
    SampleSkill { name: "Axum", category: "Backend" },
    SampleSkill { name: "Actix-web", category: "Backend" },
    SampleSkill { name: "Node.js", category: "Backend" },
    SampleSkill { name: "Postgres", category: "Data" },
    SampleSkill { name: "SQLite", category: "Data" },
    SampleSkill { name: "Redis", category: "Data" },
    SampleSkill { name: "Docker", category: "DevOps" },
    SampleSkill { name: "GitHub Actions", category: "DevOps" },
    SampleSkill { name: "Linux", category: "DevOps" },
];

pub const EXPERIENCE: &[SampleExperience] = &[
    SampleExperience {
        role: "Full Stack Developer",
        company: "Freelance",
        period: "2024 — Present",
        description: "Building Rust + TypeScript products end-to-end. Focus on clean architecture, fast iteration, and shipping things that work.",
    },
    SampleExperience {
        role: "Web Developer",
        company: "Independent Projects",
        period: "2020 — 2024",
        description: "Designed and shipped frontends with React + Svelte, plus a sprinkle of Node.js APIs and database tuning.",
    },
    SampleExperience {
        role: "Computer Science Education",
        company: "Self-taught + university coursework",
        period: "2018 — Ongoing",
        description: "Constantly learning — currently deep on Rust, distributed systems, and the boring-but-important parts of software design.",
    },
];

pub fn find_project(slug: &str) -> Option<&'static SampleProject> {
    PROJECTS.iter().find(|p| p.slug == slug)
}

pub fn find_post(slug: &str) -> Option<&'static SamplePost> {
    POSTS.iter().find(|p| p.slug == slug)
}
