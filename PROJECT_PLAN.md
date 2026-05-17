# 📋 প্রজেক্ট প্ল্যান: Portfolio (Rust + Leptos)

> বর্তমান static HTML/CSS পোর্টফোলিওকে Rust + Leptos দিয়ে আধুনিক fullstack web app-এ রূপান্তর করা হবে, যেখানে একটি admin dashboard থেকে সব content (about, projects, blog, contact ইত্যাদি) update করা যাবে এবং পুরোটা ফ্রি হোস্টিং-এ deploy করা হবে।

---

## ১. মূল লক্ষ্য (Goals)

- ✅ বর্তমান static portfolio-কে Rust + Leptos fullstack app-এ migrate করা
- ✅ Admin dashboard বানানো — যেখান থেকে সব content (text, image, project list) edit/add/delete করা যাবে
- ✅ Database-driven content (hardcoded HTML নয়)
- ✅ Authentication থাকবে — শুধু admin login করতে পারবে dashboard-এ
- ✅ সম্পূর্ণ ফ্রি hosting (কোনো credit card লাগবে না, বা শুধু verification-এ)
- ✅ Custom domain support (optional, future)
- ✅ Fast, SEO-friendly (SSR mode)

---

## ২. টেকনোলজি স্ট্যাক (Tech Stack)

### Frontend + Backend (Fullstack)
| Layer | Technology | কেন? |
|---|---|---|
| Web Framework | **Leptos** (Rust) | Modern reactive Rust framework, SSR + CSR support |
| Rendering | **Leptos SSR** (Axum backend) | SEO + dashboard-এর জন্য server logic দরকার |
| Styling | **TailwindCSS** | বর্তমান bootstrap css-এর জায়গায় modern utility-first CSS |
| Forms | `leptos_router` + `server_fn` | Type-safe form handling |

### Backend Services
| Service | Tool |
|---|---|
| Database | **SQLite** (development) → **PostgreSQL** (production) |
| ORM | **SeaORM** অথবা **SQLx** |
| Auth | **JWT** + `argon2` password hashing |
| Image upload | **Cloudinary free tier** অথবা **Cloudflare R2** |
| Email (contact form) | **Resend free tier** (3000 email/মাস) |

### Tooling
- `cargo-leptos` — Leptos build tool
- `trunk` — WASM bundling
- `sqlx-cli` — DB migrations

---

## ৩. আর্কিটেকচার (Architecture)

```
┌─────────────────────────────────────────────────┐
│              ব্রাউজার (Visitor)                  │
│  - Homepage, About, Projects, Blog, Contact     │
└────────────────────┬────────────────────────────┘
                     │ HTTPS
                     ▼
┌─────────────────────────────────────────────────┐
│        Leptos SSR App (Axum server)             │
│  ┌──────────────┐    ┌────────────────────┐    │
│  │ Public Routes│    │  Admin Dashboard   │    │
│  │ /            │    │  /admin/login      │    │
│  │ /about       │    │  /admin/dashboard  │    │
│  │ /projects    │    │  /admin/projects   │    │
│  │ /blog        │    │  /admin/blog       │    │
│  │ /contact     │    │  /admin/settings   │    │
│  └──────────────┘    └────────────────────┘    │
│           │                    │                │
│           └──────┬─────────────┘                │
│                  ▼                              │
│         ┌────────────────┐                      │
│         │  Server Fns    │                      │
│         │  (CRUD logic)  │                      │
│         └────────┬───────┘                      │
└──────────────────┼──────────────────────────────┘
                   ▼
        ┌──────────────────────┐
        │  Database (Postgres) │
        │  + Cloudinary (img)  │
        └──────────────────────┘
```

---

## ৪. মূল ফিচারসমূহ (Features)

### A. পাবলিক সাইট (Visitor-facing)
- [ ] Homepage — hero, intro, featured projects
- [ ] About page — bio, skills, experience
- [ ] Projects page — dynamic list (DB থেকে আসবে)
- [ ] Project detail page — `/projects/<slug>`
- [ ] Blog list page
- [ ] Blog detail page — Markdown rendering
- [ ] Contact form — message DB-তে save + email notification
- [ ] Dark/Light mode toggle
- [ ] Responsive design (mobile + tablet + desktop)
- [ ] SEO meta tags (dynamic per page)
- [ ] Sitemap.xml + robots.txt

### B. অ্যাডমিন ড্যাশবোর্ড (Dashboard)
- [ ] Login page (email + password)
- [ ] Dashboard home — quick stats (visits, messages, total projects)
- [ ] **Profile editor** — name, bio, photo, social links
- [ ] **Projects manager** — Create / Edit / Delete / Reorder
  - Title, description, image, tech tags, GitHub link, live link
- [ ] **Blog manager** — Markdown editor + preview
  - Title, slug, content, cover image, tags, publish/draft
- [ ] **Skills manager** — add/remove skill chips
- [ ] **Messages inbox** — contact form-এর message দেখা + reply
- [ ] **Site settings** — site title, SEO defaults, footer text
- [ ] Image upload (drag-drop)
- [ ] Logout

---

## ৫. ড্যাশবোর্ড ডিটেইল (Dashboard Design)

```
┌──────────────────────────────────────────────────┐
│  📊 Dashboard            [Profile] [Logout]      │
├──────────┬───────────────────────────────────────┤
│ Sidebar  │  Main Content Area                    │
│          │                                       │
│ 🏠 Home  │  ┌─ Quick Stats ──┐                  │
│ 👤 Profile│  │ 12 Projects     │                 │
│ 💼 Projects│  │ 5 Blog Posts   │                 │
│ ✍️  Blog  │  │ 23 Messages    │                 │
│ 🛠️  Skills │  └─────────────────┘                │
│ 📬 Inbox │                                       │
│ ⚙️  Settings│  Recent Activity...                │
│          │                                       │
└──────────┴───────────────────────────────────────┘
```

---

## ৬. ডাটাবেস স্কিমা (Database Schema)

```sql
-- Users (admin)
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT,
    created_at TIMESTAMP
);

-- Profile (single row)
CREATE TABLE profile (
    id INT PRIMARY KEY,
    name TEXT,
    title TEXT,
    bio TEXT,
    photo_url TEXT,
    email TEXT,
    github TEXT,
    linkedin TEXT,
    twitter TEXT
);

-- Projects
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT UNIQUE,
    description TEXT,
    image_url TEXT,
    tech_stack TEXT[],
    github_url TEXT,
    live_url TEXT,
    featured BOOLEAN DEFAULT FALSE,
    display_order INT,
    created_at TIMESTAMP
);

-- Blog posts
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    title TEXT,
    slug TEXT UNIQUE,
    content_md TEXT,
    cover_image TEXT,
    tags TEXT[],
    published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMP,
    created_at TIMESTAMP
);

-- Skills
CREATE TABLE skills (
    id UUID PRIMARY KEY,
    name TEXT,
    icon TEXT,
    category TEXT,
    display_order INT
);

-- Contact messages
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    name TEXT,
    email TEXT,
    subject TEXT,
    body TEXT,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP
);
```

---

## ৭. ফেইজ ভিত্তিক প্ল্যান (Phases / Milestones)

### 🚀 Phase 1: Foundation (Week 1)
- [ ] `cargo-leptos new portfolio` দিয়ে project scaffold
- [ ] TailwindCSS integrate
- [ ] Routing setup (`leptos_router`)
- [ ] Basic layout (Navbar, Footer)
- [ ] বর্তমান static HTML থেকে content copy কর‍ে hardcoded version বানানো (visual parity)

### 🎨 Phase 2: Public Pages UI (Week 2)
- [ ] Homepage + Hero section
- [ ] About page
- [ ] Projects list + detail page (এখনো hardcoded data)
- [ ] Blog list + detail page (Markdown rendering)
- [ ] Contact page (form only)
- [ ] Dark/Light mode
- [ ] Responsive testing

### 🗄️ Phase 3: Database + Server Functions (Week 3)
- [ ] SQLx setup + migrations
- [ ] CRUD server functions for projects/posts/skills
- [ ] Hardcoded content → DB-driven এ shift
- [ ] Contact form → DB save + email notification (Resend)

### 🔐 Phase 4: Auth + Dashboard (Week 4)
- [ ] User table + signup (one-time admin create)
- [ ] Login + JWT cookie session
- [ ] Protected `/admin/*` routes
- [ ] Dashboard layout
- [ ] Profile editor
- [ ] Projects manager (CRUD UI)
- [ ] Blog manager + Markdown editor
- [ ] Image upload (Cloudinary integration)
- [ ] Messages inbox

### 🌐 Phase 5: Polish + SEO (Week 5)
- [ ] Meta tags per page (dynamic)
- [ ] Sitemap generator
- [ ] OG image generation
- [ ] Loading states + error boundaries
- [ ] Form validation feedback
- [ ] Accessibility audit (keyboard nav, aria)
- [ ] Performance: image lazy load, caching

### 🚢 Phase 6: Deployment (Week 6)
- [ ] Production build (`cargo leptos build --release`)
- [ ] Environment variables setup
- [ ] Database migration on production
- [ ] CI/CD: GitHub Actions → auto-deploy on push to main
- [ ] Domain + HTTPS
- [ ] Analytics (privacy-friendly: Plausible/Umami self-hosted, না হলে Cloudflare Web Analytics free)

---

## ৮. ফ্রি হোস্টিং অপশন (Free Hosting)

Rust+Leptos SSR app + PostgreSQL host করার জন্য সবচেয়ে ভালো ফ্রি অপশন:

### 🥇 অপশন ১ (Recommended): **Shuttle.rs**
- Rust-native hosting
- Built-in PostgreSQL (free)
- `cargo shuttle deploy` দিয়েই deploy
- Free tier: 1 project, 0.5 vCPU, যথেষ্ট portfolio-এর জন্য
- কোনো credit card লাগে না

### 🥈 অপশন ২: **Fly.io**
- Generous free tier (3 small VMs)
- Postgres ক্লাস্টার free tier ছিল, এখন paid (alternative: Supabase free DB)
- Docker-based deploy
- Credit card verification লাগে কিন্তু free tier-এ charge হয় না

### 🥉 অপশন ৩: **Cloudflare Workers + D1**
- Leptos-কে WASM হিসেবে edge-এ চালানো (experimental)
- D1 = serverless SQLite, 5GB free
- 100k requests/day free
- সবচেয়ে fast (edge network)
- কিন্তু Leptos SSR full support এখনো maturing

### 🆓 ফ্রি Database Alternative
- **Supabase** — 500MB Postgres free forever
- **Neon** — 0.5GB serverless Postgres free
- **Turso** — 9GB SQLite free (LibSQL, edge-replicated)

### 📦 ফ্রি ইমেজ স্টোরেজ
- **Cloudinary** — 25 credits/মাস free
- **Cloudflare R2** — 10GB storage free, no egress fee
- **ImageKit** — 20GB bandwidth free

### 🎯 চূড়ান্ত সুপারিশ
**Shuttle.rs (app + DB) + Cloudflare R2 (images) + Resend (email)** — সম্পূর্ণ ফ্রি, scalable, এবং Rust-friendly।

---

## ৯. কী কী শিখতে হবে (Learning Path)

যদি Leptos নতুন হয়:
1. [Leptos Book](https://book.leptos.dev/) — official tutorial
2. Server functions concept (`#[server]` macro)
3. Signals + reactivity (`create_signal`, `create_resource`)
4. `view!` macro
5. Routing with `leptos_router`
6. SSR + hydration লাইফসাইকেল

---

## ১০. পরবর্তী পদক্ষেপ (Next Steps)

আজকেই শুরু করতে চাইলে:

```bash
# 1. cargo-leptos install
cargo install cargo-leptos

# 2. Project scaffold
cd ~/Downloads
cargo leptos new portfolio-leptos --git leptos-rs/start-axum
cd portfolio-leptos

# 3. Run dev server
cargo leptos watch
```

তারপর `http://localhost:3000` এ গিয়ে starter app দেখা যাবে।

---

## ১১. ঝুঁকি ও বিবেচনা (Risks & Considerations)

| ঝুঁকি | কারণ | সমাধান |
|---|---|---|
| Leptos এখনো 0.x version | API change হতে পারে | Specific version pin করব (`leptos = "=0.6.x"`) |
| Free tier limits | Traffic বেশি হলে throttle | Cloudflare CDN ক্যাশিং front-এ |
| Cold start (Shuttle) | প্রথম request slow | Health-check ping cron |
| Markdown XSS | User content render | `ammonia` crate দিয়ে sanitize |

---

## ১২. সাফল্যের মাপকাঠি (Success Criteria)

- ✅ `https://azharcse14.dev` (বা equivalent) তে live
- ✅ Dashboard থেকে নতুন project add করলে instantly সাইটে দেখা যায়
- ✅ Lighthouse score: Performance 90+, SEO 95+, Accessibility 95+
- ✅ Initial page load < 2 seconds
- ✅ Monthly cost: **৳০**

---

**তৈরি:** ২০২৬-০৫-১৮
**লেখক:** Azharul Islam (@azharcse14)
**রেপো:** https://github.com/azharcse14/portfolio-rust
