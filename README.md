# Thailand Football

[![Live](https://img.shields.io/badge/Live-online-brightgreen?style=flat-square)](https://thailand-football.onrender.com)
[![Rust](https://img.shields.io/badge/Rust-stable-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-6E4AFF?style=flat-square)](https://docs.rs/axum/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Render](https://img.shields.io/badge/Render-Free-46E3B7?style=flat-square&logo=render&logoColor=black)](https://render.com/)
[![Neon](https://img.shields.io/badge/Neon-PostgreSQL-00E599?style=flat-square&logo=neon&logoColor=black)](https://neon.com/)

Public directory of Thai football competitions, teams, seasons, and verified roster sources.

**Live website:** <https://thailand-football.onrender.com>

## Overview

Thailand Football presents Thai football competitions in one place: organizer groups, competitions, seasons, age/type dimensions, team lists, standings, cup rounds, and source-backed roster information.

The application is a single Rust monolith. The production site runs in PostgreSQL mode on Render Free with Neon PostgreSQL.

As of 26 November 2026 the directory contains 18 active competitions, 196 permanent teams, and 371 season participations.

The directory hierarchy is:

**Organizer Group → Competition → Season → Age Group / Type → Teams → Fixtures / Results**

## Competition groups

### National League

- Thai League 1
- Thai League 2
- Thai League 3
- Thai FA Cup
- Thai League Cup
- Thai Champion Cup

### Youth League (FAT & SAT)

- Thai Youth League 1 (U18, U16, U14)
- Thai Youth League 2 (U18, U16, U14)
- Thai Youth League 3 (U18, U16, U14)
- Thai Youth FA Cup (U17) - Coke Cup
- Thai Youth League Cup (U18) - 7HD Champion Cup
- Thai Youth Champion Cup (U18) - Jaturamitr Samakkhi

U18/U16/U14 are season-level age dimensions within each Thai Youth League parent competition, matching the School League pattern.

### School League (DPE)

- Thai School League — Type A (U18, U16, U14)
- Thai School League — Type B (U18, U16, U14)
- Thai School League — General (U18, U16, U14, U12)
- Thai College Cup (U19)
- Thai University Cup (U23)
- Thai School Super Cup (U23) - Thammasat–Chulalongkorn tradition

Type A, Type B, and General are separate competition identities. U18/U16/U14, plus U12 for General, are season-level age dimensions within each school competition.

### Cup coverage (current snapshot)

- Thai FA Cup 2025/26: 7 rounds, 94 ties; champion Buriram United, runner-up PT Prachuap.
- Thai League Cup 2025/26: 8 rounds, 79 ties; champion Port FC, runner-up BG Pathum United.
- Thai College Cup 2026: 8 teams and 7 knockout ties across quarter-finals, semi-finals, and final; champion Thawee SC.
- Thai Youth FA Cup (Coke Cup U17) 2026: quarter-finals through final; champion Assumption United.
- Thai Youth League Cup (7HD Champion Cup U18) 2026: quarter-final draw recorded; matches start 30 November 2026.
- Thai Youth Champion Cup (Jaturamitr Samakkhi) 2025: full matchdays, third-place match, and final; champion Debsirin School.
- Thai University Cup (Chang U-Champion Cup) 2026: quarter-finals through final; champion Thailand National Sports University.
- Thai School Super Cup (U23): 75th Thammasat–Chulalongkorn match, 15 February 2025; Thammasat University 2–1 Chulalongkorn University.

Cup rounds use the original vertical round list, with match dates, scores, penalty shootouts, extra time, unavailable-score labels, and advancing teams. Large early qualification rounds remain collapsible.

## Main features

- Competition, season, age-group, and type navigation
- Standings, results, fixtures, cup rounds, team statistics, and top scorers where records exist
- Separate penalty-shootout and extra-time metadata, with honest unavailable-score labels
- Team-first lists with source links, verification badges, and roster-season metadata
- Stable team-ID search with competition, group, and season context
- Favorites and light/dark theme persisted in the browser
- Honest empty states when no public team list or verified records exist
- Health endpoint at `/healthz`

## Data verification policy

Team identities and season participations are stored separately. A team participation records the competition, season, age/type, source URL, publisher, source date, verification status, and roster metadata.

Verification statuses are `verified_official`, `verified_secondary`, `discovered_unverified`, `conflicting`, and `rejected`. Imported values are preserved; standings remain unverified unless independently checked.

Roster labels used in the Teams tab:

| Label | Meaning |
|---|---|
| `current` | A verified current-season roster. |
| `latest_verified` | The current-season roster is unavailable; the latest complete verified previous-season roster is shown with its displayed and source season. |
| `partial` | Only a documented subset is available, such as a final round, confirmed knockout participants, or an unverified public preview. |
| `unavailable` | No reliable current-season or previous-season roster exists; the Teams tab keeps an honest empty state. |

Fallback and partial rosters are reference-only. They are never inserted into current-season standings, fixtures, results, or statistics.

The project does not fabricate teams, fixtures, results, scorers, or champions. Repeated names do not automatically prove the same team, and reserve, school, club, and age-group identities remain separate unless source-based curation supports a merge.

## Technology stack

- Rust stable, edition 2024
- Axum and Tokio
- Askama templates
- SQLx and PostgreSQL
- Neon PostgreSQL in production
- Tailwind CSS v4
- Minimal vanilla JavaScript and HTMX where useful
- Render Free for the web service

## Production deployment

- **Live website:** <https://thailand-football.onrender.com>
- **Hosting:** Render Free
- **Database:** Neon PostgreSQL
- **Mode:** PostgreSQL
- **Health check:** `/healthz`

The service runs SQLx migrations and imports the seed data and team registry before serving production data. Deployment configuration is managed outside this repository.

Render Free may sleep during inactivity. The first request after an idle period may take roughly a minute while the service wakes up.

The deployment runbook is in [`DEPLOYMENT.md`](DEPLOYMENT.md).

## Local development

Prerequisites: Rust stable, Node.js with npm, and optionally a PostgreSQL database.

```powershell
# Install frontend build tooling
npm install

# Build the production stylesheet
npm run css:build

# Format, check, and test
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets

# Build and run
cargo build
.\target\debug\thailand-football.exe
```

Open <http://127.0.0.1:8080>. If port 8080 is occupied, the application tries ports 8080–8089 and logs the chosen URL.

Without a database connection configured, the application runs as a read-only local preview from `data/seed.json` and shows a `Local preview` badge. With a PostgreSQL connection configured, it uses the database-backed path. See [`DEPLOYMENT.md`](DEPLOYMENT.md) and [`.env.example`](.env.example) for configuration guidance.

On Windows GNU builds, ensure the MinGW `gcc` and `dlltool` tools are available on `PATH`. The compiled binary does not require MinGW at runtime.
