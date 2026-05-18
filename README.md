# Portfolio · Rust + Leptos

Personal portfolio + admin dashboard built on a Cargo workspace with clean architecture (domain / application / infrastructure / presentation). Server-rendered with [Leptos](https://leptos.dev/) on Axum, SQLite via SQLx, JWT-cookie auth, and an admin panel for editing every dynamic section.

- **Live:** _set after first deploy_
- **Repo:** https://github.com/azharcse14/portfolio-rust
- **Spec:** [PROJECT_PLAN.md](./PROJECT_PLAN.md) (Bangla)
- **Progress:** [PROGRESS.md](./PROGRESS.md)

---

## Stack

| Layer            | Choice                                                              |
| ---------------- | ------------------------------------------------------------------- |
| Web framework    | Leptos 0.7 (SSR + WASM hydration)                                   |
| HTTP             | Axum 0.7 + tower-http (compression, cache headers, static serving)  |
| Database         | SQLite via SQLx 0.8 (migrations baked into the binary)              |
| Auth             | argon2id password hashing + HS256 JWT (HttpOnly + SameSite cookie)  |
| Frontend (admin) | Hand-rolled CSS, scoped under `#admin-app`                          |
| Frontend (site)  | Legacy Jackson template (Bootstrap 3 + jQuery)                      |
| Markdown         | `pulldown-cmark` server-rendered                                    |

---

## Local development

```bash
# 1. Install Rust (stable) and cargo-leptos
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-leptos --locked
rustup target add wasm32-unknown-unknown

# 2. Run the app — auto-creates portfolio.db with seed content
cargo leptos serve

# Site:           http://127.0.0.1:3000/
# Admin setup:    http://127.0.0.1:3000/admin/setup
# Admin login:    http://127.0.0.1:3000/admin/login
```

The dev server watches `crates/**/*.rs` and the Tailwind file; pages auto-reload.

---

## Production deployment (Fly.io)

Free-tier hosting on Fly.io. Singapore region (`sin`) for low latency from South Asia.

### One-time setup

```bash
# 1. Install flyctl
curl -L https://fly.io/install.sh | sh

# 2. Sign in (one-time)
flyctl auth signup     # or: flyctl auth login

# 3. Create the app (uses fly.toml in repo root)
flyctl launch --no-deploy --copy-config

# 4. Create the persistent SQLite volume — sizing is generous; SQLite is small
flyctl volumes create portfolio_data --region sin --size 1

# 5. Set production secrets. Generate JWT_SECRET with `openssl rand -hex 64`.
flyctl secrets set JWT_SECRET="$(openssl rand -hex 64)"

# 6. Deploy
flyctl deploy
```

After the first deploy, `flyctl deploy` (or pushing to `main` if CI/CD secret is configured) ships new code.

### Continuous deployment via GitHub Actions

The `.github/workflows/deploy.yml` workflow auto-deploys on every push to `main`. To enable it:

1. `flyctl tokens create deploy -x 999999h` → copy the printed token.
2. In the GitHub repo: **Settings → Secrets and variables → Actions → New repository secret**:
   - Name: `FLY_API_TOKEN`
   - Value: paste the token from step 1.
3. Push to `main`. The workflow runs `flyctl deploy --remote-only` and waits for healthy.

### First-time admin bootstrap (after deploy)

1. Visit `https://<your-app>.fly.dev/admin/setup`
2. Create the admin account (email + 8+ char password).
3. You're auto-logged in. Edit content under `/admin/profile`, `/admin/projects`, `/admin/posts`, `/admin/skills`, `/admin/messages`.

`/admin/setup` refuses once any admin exists — the route closes itself after first use.

---

## Alternative: Shuttle.rs

If you'd rather skip the Docker workflow and Fly's credit-card verification, [Shuttle.rs](https://www.shuttle.rs/) hosts Rust apps with one command.

Migration sketch (not yet wired into this repo):

```rust
// crates/presentation/src/main.rs — replace #[tokio::main] block with:
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // existing setup, return Router instead of running serve
    Ok(app.into())
}
```

Plus add to `crates/presentation/Cargo.toml`:

```toml
shuttle-axum = "0.x"
shuttle-runtime = "0.x"
```

Then `cargo shuttle deploy`.

---

## Environment variables

| Variable          | Default                                | Required in prod |
| ----------------- | -------------------------------------- | ---------------- |
| `DATABASE_URL`    | `sqlite://portfolio.db?mode=rwc`       | yes              |
| `JWT_SECRET`      | `dev-secret`                           | **yes**          |
| `LEPTOS_SITE_ADDR`| `127.0.0.1:3000` (dev) / `0.0.0.0:8080`| recommended      |
| `RUST_LOG`        | `info`                                 | optional         |

See [.env.example](./.env.example).

---

## Architecture

See [PROJECT_PLAN.md](./PROJECT_PLAN.md) for the full picture. The short version:

```
crates/
├── domain/          ← entities + value objects + repository traits (zero I/O)
├── application/     ← use-cases (depend only on domain)
├── infrastructure/  ← SQLx adapters, argon2, JWT, config (impls of domain ports)
└── presentation/    ← Leptos + Axum, server fns, composition root, routes
```

Dependency rule: outer crates depend on inner ones, never the reverse. The
public site and the admin dashboard are both wired through the same use-case
layer; the only thing that differs is who's allowed to call which fn (auth
guard checks the JWT cookie before any admin server fn touches the DB).

---

## License

MIT.
