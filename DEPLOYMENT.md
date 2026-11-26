# Production deployment

Approved stack: private GitHub repository, Rust web service on Render Free, and Neon Free PostgreSQL.

Never commit `DATABASE_URL`, passwords, API keys, or tokens.

## 1. GitHub

The private repository is:

`https://github.com/nattapongsindhu/thailand-football`

Only project files are committed. `target/`, `node_modules/`, `logs/`, `.env`, and `.env.local` are ignored.

## 2. Neon PostgreSQL

Create one Neon project/database, for example `thailand-football`, and copy its pooled or direct connection string.

Set it only in the Render environment, never in this repository:

```text
DATABASE_URL=postgres://USER:PASSWORD@HOST/DBNAME?sslmode=require
```

## 3. Migrations and imports

The import binaries run all SQLx migrations before inserting data.

```powershell
$env:DATABASE_URL = '<Neon connection string>'
cargo run --release --bin import_seed
cargo run --release --bin import-team-registry
```

After import, set the runtime mode so the application reports PostgreSQL rather than local preview:

```sql
UPDATE app_meta SET value = 'postgresql' WHERE key = 'mode';
```

The team registry (including roster fallback metadata) is loaded from `data/team_registry.json` at runtime, so that file must be deployed with the service. The database import also stores team identities and participations.

## 4. Render

`render.yaml` defines one free Rust web service:

- Build: `cargo build --release --locked`
- Start: `./target/release/thailand-football`
- Health check: `/healthz`
- Runtime: Rust (latest stable)
- Plan: Free

Render provides `PORT`. The application binds `0.0.0.0:$PORT` when `PORT` is set and falls back to `127.0.0.1:8080-8089` locally. Set `DATABASE_URL` in the Render dashboard as a secret.

## 5. Verification

After deployment:

```text
GET https://<service>.onrender.com/healthz
```

Expected: `status: ok`, `mode: postgresql`, 17 competitions, and a non-zero search/index count.

Also verify:

- National League teams page.
- Youth Teams tab.
- School Teams tab.
- `latest_verified` roster badge/notice.
- Empty-state categories.
- No fallback teams in standings, results, fixtures, or statistics.

## 6. Secrets

Do not place any credential in `render.yaml`, `.env.example`, source code, tests, data files, or Git history. Use the Render environment variable `DATABASE_URL` and Neon's own secret store.
