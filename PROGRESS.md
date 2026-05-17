# 📊 প্রজেক্ট প্রোগ্রেস ট্র্যাকার

> **পূর্ণ স্পেক:** [PROJECT_PLAN.md](./PROJECT_PLAN.md)
> **শেষ আপডেট:** 2026-05-18

---

## 🎯 সামগ্রিক অগ্রগতি (Overall)

```
[░░░░░░░░░░░░░░░░░░░░] 0% complete  (0/6 phases)
```

| Phase | Status | শুরু | শেষ | নোট |
|---|---|---|---|---|
| 0. Setup & Planning | 🟢 Done | 2026-05-18 | 2026-05-18 | Repo + plan ready |
| 1. Foundation (workspace, scaffold, Tailwind) | ⚪ Not started | — | — | — |
| 2. Public Pages UI | ⚪ Not started | — | — | — |
| 3. Database + Server Functions | ⚪ Not started | — | — | — |
| 4. Auth + Dashboard | ⚪ Not started | — | — | — |
| 5. Polish + SEO | ⚪ Not started | — | — | — |
| 6. Deployment | ⚪ Not started | — | — | — |

**Status icons:** 🟢 Done · 🟡 In progress · 🔴 Blocked · ⚪ Not started

---

## 🚀 বর্তমান ফোকাস (Current Focus)

**Phase:** —
**Task:** —
**Blocker:** কোনো blocker নেই

---

## ✅ Phase 0: Setup & Planning

- [x] GitHub repo create (`azharcse14/portfolio-rust`)
- [x] `gh` CLI install + auth setup
- [x] Initial commit (static HTML baseline)
- [x] `PROJECT_PLAN.md` লেখা
- [x] Clean Architecture section + folder tree
- [x] `PROGRESS.md` ট্র্যাকার তৈরি

---

## ⬜ Phase 1: Foundation

**Goal:** Cargo workspace + 4-crate clean architecture skeleton + Leptos SSR + TailwindCSS dev server চলবে।

- [ ] `cargo install cargo-leptos`
- [ ] Workspace root `Cargo.toml` তৈরি
- [ ] `crates/domain` crate (lib only)
  - [ ] `entities/` placeholder modules
  - [ ] `ports/` repository traits
  - [ ] `errors.rs`
- [ ] `crates/application` crate (depends on domain)
- [ ] `crates/infrastructure` crate (depends on domain + application)
- [ ] `crates/presentation` crate — Leptos SSR + Axum bootstrap
- [ ] TailwindCSS integration
- [ ] Basic layout components (Navbar, Footer)
- [ ] `cargo leptos watch` কাজ করছে confirm
- [ ] CI: `cargo check --workspace` GitHub Action

---

## ⬜ Phase 2: Public Pages UI

**Goal:** সব visible page-এর UI ready, content এখনো hardcoded।

- [ ] Homepage (hero + intro + featured projects placeholder)
- [ ] About page
- [ ] Projects list page
- [ ] Project detail page (`/projects/<slug>`)
- [ ] Blog list page
- [ ] Blog detail page (Markdown rendering)
- [ ] Contact page (form UI only)
- [ ] Dark / Light mode toggle
- [ ] Responsive (mobile / tablet / desktop)
- [ ] 404 + 500 error pages

---

## ⬜ Phase 3: Database + Server Functions

**Goal:** সব content DB থেকে আসবে, hardcoded data সরবে।

- [ ] SQLx + Postgres dev setup (Docker compose বা local)
- [ ] Migrations folder + initial schema (6 tables)
- [ ] `domain::ports` trait define
- [ ] `infrastructure` repo impl (SQLx)
- [ ] Use-cases in `application` (list/get/create/update/delete)
- [ ] Server functions in `presentation/server_fns/`
- [ ] Public pages → DB-driven content shift
- [ ] Contact form → DB save + Resend email
- [ ] Use-case unit tests (mock repo)
- [ ] Repo integration tests (`sqlx::test`)

---

## ⬜ Phase 4: Auth + Dashboard

**Goal:** Admin login করে dashboard থেকে সব content manage করতে পারবে।

- [ ] User table + argon2 password hashing
- [ ] Login server function + JWT cookie session
- [ ] `auth_guard` middleware for `/admin/*`
- [ ] Admin layout + sidebar
- [ ] Profile editor page
- [ ] Projects manager (CRUD UI + reorder)
- [ ] Blog manager + Markdown editor + preview
- [ ] Skills manager
- [ ] Image upload (Cloudinary integration)
- [ ] Messages inbox + mark as read
- [ ] Site settings page
- [ ] Logout flow

---

## ⬜ Phase 5: Polish + SEO

**Goal:** Production quality — fast, accessible, discoverable।

- [ ] Dynamic meta tags per page (title, description, OG)
- [ ] Sitemap.xml generator
- [ ] robots.txt
- [ ] OG image generation
- [ ] Loading skeletons + error boundaries
- [ ] Form validation feedback (inline)
- [ ] Accessibility audit (axe-core, keyboard nav, aria)
- [ ] Image lazy loading
- [ ] Server-side cache headers
- [ ] Lighthouse score ≥ 90 (Perf/SEO/A11y)

---

## ⬜ Phase 6: Deployment

**Goal:** Live URL-এ public access, auto-deploy on push।

- [ ] Production build verify (`cargo leptos build --release`)
- [ ] Env vars documented (`.env.example`)
- [ ] Hosting choose (Shuttle.rs / Fly.io)
- [ ] Production DB provision (Shuttle Postgres / Supabase / Neon)
- [ ] Cloudinary account + API keys
- [ ] Resend account + domain verify
- [ ] First deploy successful
- [ ] GitHub Actions CI/CD pipeline (push to main → deploy)
- [ ] Domain + HTTPS (or default subdomain)
- [ ] Analytics (Cloudflare Web Analytics free)
- [ ] Backup strategy (DB dump cron)

---

## 📝 আপডেট লগ (Changelog)

প্রতিটা significant change এখানে এক লাইনে লেখো (latest উপরে)।

- **2026-05-18** — Repo + plan + progress tracker setup ✅

---

## 🔧 কিভাবে আপডেট করব (Workflow)

প্রতিবার একটা task শেষ হলে:

1. উপরের checkbox `[ ]` → `[x]` করো
2. Phase status icon update (⚪ → 🟡 in progress, 🟡 → 🟢 done)
3. **Current Focus** section update
4. Progress bar manually update — `[██░░░░] 33%` ধরন
5. **Changelog** এ এক লাইন add
6. Commit message format: `progress: <কী হলো>`

**উদাহরণ commit:**
```
git commit -m "progress: phase 1 - workspace scaffold + domain crate done"
```

পুরা phase শেষ হলে:
```
git tag phase-1-complete
git push --tags
```

---

## 📌 ব্লকার / প্রশ্ন (Open Questions)

> এখানে এমন কিছু লেখো যা decide করা বাকি বা stuck আছে।

- কোন hosting final — Shuttle.rs না Fly.io? (decision Phase 6-এ)
- Production DB: Shuttle Postgres নাকি Supabase? (5000 row range — দুটোই enough)
- Custom domain কিনব কিনা? (free `.dev` subdomain দিয়ে শুরু)
