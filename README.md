# Thailand Football

Independent Rust monolith for Thai football competitions, built from the owner-supplied offline reference application and a new, owner-approved visual direction.

**Local preview:** `http://127.0.0.1:8080`

> Production deployment and GitHub push are not authorized in this phase.

## Technology

- Rust stable, edition 2024
- Axum + Tokio
- Askama templates
- SQLx + PostgreSQL (schema and import path ready)
- Tailwind CSS v4 (compiled at build time)
- Minimal vanilla JavaScript
- HTMX for the team-search dropdown

## Implemented

- Exact owner-approved organizer-group sidebar: National League, Youth League, and School League
- Thai League 1/2/3 standings, including T3 regional groups
- Results and fixtures where imported records exist
- Team statistics and top scorers
- Thai FA Cup, Thai League Cup and Thai Champion Cup round/ties views
- Existing news view, filtered by competition with no senior-data fallback
- Stable team-ID search with competition/group/season context
- Stable team-ID favorites stored in `localStorage`
- Light/dark theme stored in `localStorage`
- Responsive sidebar and tables
- Honest empty states for owner-defined youth categories
- Team-first competition team lists with verification badges, source links, age/type context, and last-updated timestamps
- Discovery source registry and repeatable local update command
- News discovery metadata with provenance and extracted team IDs
- Health endpoint: `/healthz`
- Local preview mode with a visible `Local preview` badge

### Sidebar

**National League**

- Thai League 1
- Thai League 2
- Thai League 3
- Thai FA Cup
- Thai League Cup
- Thai Champion Cup

**Youth League (FAT & SAT)**

- Thai Youth League 1 (U18)  
  *(ฟุตบอลเยาวชน สมาคมฯ ลีก 1)*
- Thai Youth League 2 (U16)  
  *(ฟุตบอลเยาวชน สมาคมฯ ลีก 2)*
- Thai Youth League 3 (U14)  
  *(ฟุตบอลเยาวชน สมาคมฯ ลีก 3)*
- Thai Youth FA Cup (U17)  
  *(ฟุตบอลเยาวชน โค้กคัพ Coke Cup)*  
  Alias / sponsor name: **Coke Cup**
- Thai Youth League Cup (U18)  
  *(ฟุตบอลเยาวชน แชมป์กีฬา 7HD)*  
  Alias / sponsor name: **7HD Champion Cup**
- Thai Youth Champion Cup (U18)  
  *(ฟุตบอลเยาวชน จตุรมิตรสามัคคี)*

**School League (DPE)**

- Thai School League — Type A
  - U18
  - U16
  - U14
  - *(ฟุตบอลนักเรียน กรมพลฯ ประเภท ก)*
- Thai School League — Type B
  - U18
  - U16
  - U14
  - *(ฟุตบอลนักเรียน กรมพลฯ ประเภท ข)*
- Thai School League — General
  - U18
  - U16
  - U14
  - U12
  - *(ฟุตบอลนักเรียน กรมพลฯ ประเภท ทั่วไป)*
- Thai College Cup (U19)  
  *(ฟุตบอลอาชีวะระดับประเทศ)*
- Thai University Cup (U23)  
  *(ฟุตบอลมหาวิทยาลัยระดับประเทศ)*

Thai League 4 and Thai League 5 are preserved as inactive legacy data in `data/legacy_t4_t5.json`. They are not active sidebar entries and are not routed.

### Data hierarchy

Organizer Group → Competition → Season → Age Group / Type → Teams → Fixtures / Results

For school competitions, Type A / Type B / General are competition identities, while U18/U16/U14/U12 are season-level age dimensions selectable beneath each competition.

## Local preview mode

If `DATABASE_URL` is not set, the application runs as an explicit **read-only local preview**:

- Data is loaded from `data/seed.json`.
- The UI shows a `Local preview` badge.
- No persistent writes are simulated.
- PostgreSQL migrations and a controlled import path remain ready for later setup.
- SQLite is not used.

If `DATABASE_URL` is set, the application connects to PostgreSQL, runs migrations, and loads the relational data. This mode has not been runtime-tested because no project-specific development database was configured.

## Run locally

```powershell
# 1. Production-style CSS build
npm install
npm run css:build

# 2. Rust checks
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets

# 3. Build and run
cargo build
.\target\debug\thailand-football.exe
```

Open `http://127.0.0.1:8080`.

The server binds only to `127.0.0.1`. If port 8080 is occupied, it tries 8080–8089 and logs the chosen URL.

### Windows toolchain note

This machine's default Rust host is `x86_64-pc-windows-gnu`. The MinGW tools are at `C:\Users\BMW\mingw64\bin`. If Cargo cannot find `gcc.exe`/`dlltool.exe`:

```powershell
$env:Path = 'C:\Users\BMW\mingw64\bin;' + $env:Path
```

The compiled binary itself does not need MinGW at runtime.

## Stop locally

Use the process ID printed by the server/reported in `PROJECT_STATUS.md`:

```powershell
Stop-Process -Id <PID>
```

Logs are written to `logs/` and are ignored by Git.

## Team discovery and news ingestion

Team-first data is stored in `data/team_registry.json` and loaded by the preview app:

- Permanent team identity: `team_id`, display/Thai/English names, aliases, type, province
- Participation: competition, season, age group, type/group, source, verification, first/last seen
- Verification: `verified_official`, `verified_secondary`, `discovered_unverified`, `conflicting`, `rejected`

Source discovery:

- `data/source_registry.json` — 17 approved competitions, organizer groups, official/secondary domains, search terms, fetch URLs, status, last checked
- `data/discovery/curated_verified_teams.json` — curated public-source participations for national leagues, cups, school age groups, youth cups, and university football
- `data/discovery/news_metadata.json` — headlines, source metadata, extracted stable team IDs; no article bodies
- `data/discovery/inbox/` — fetched-source metadata and candidate names
- `data/discovery/reports/latest.json` / `latest.md` — latest validation and discovery report

Run the repeatable local update:

```powershell
cargo run --bin update-football-data
```

Offline metadata-only run:

```powershell
$env:TF_DISCOVERY_NO_FETCH = '1'
cargo run --bin update-football-data
```

The command fetches only registered public sources, writes candidates to the inbox, applies curated verified participations, preserves existing records, and validates the result. Automatic candidates are not promoted to the team registry without curation.

The Teams tab on each competition page shows the current team list, season, age/type, verification badges, source links, and an honest empty state when no verified list exists.

This process can later be scheduled with GitHub Actions or Render after deployment approval. No cloud scheduling is active now.

## PostgreSQL / Neon setup

See `DEPLOYMENT.md` for the Render + Neon production runbook.

1. Create a project-owned PostgreSQL database (Neon or local) with a dedicated user.
2. Set `DATABASE_URL` outside the repository; never commit credentials.
3. Run the controlled import:

```powershell
$env:DATABASE_URL = 'postgres://USER:PASSWORD@HOST:5432/DBNAME?sslmode=require'
cargo run --release --bin import_seed -- data/seed.json
cargo run --release --bin import-team-registry
```

`import_seed` creates competitions/seasons/standings; `import-team-registry` imports team identities, participations, source provenance, and discovery-news metadata by stable IDs. Both binaries run all SQLx migrations before inserting data.

After import, set PostgreSQL mode:

```sql
UPDATE app_meta SET value = 'postgresql' WHERE key = 'mode';
```

4. Start the application with the same `DATABASE_URL`.

The import is idempotent for stable IDs and preserves standings snapshots by `captured_at`. The application loads the team registry from `data/team_registry.json` in both preview and PostgreSQL modes. When `PORT` is set (Render), the app binds `0.0.0.0:$PORT`; otherwise it uses `127.0.0.1:8080-8089`.

Schema areas:

- Sources and provenance
- Competitions, aliases, competition sources
- Seasons, season groups, season sources
- Teams, team aliases, participations
- Team discovery metadata and provenance
- Discovery news metadata
- Standings snapshots
- Matches
- Cup editions, rounds, ties, past editions
- Scorers
- News and news/competition links
- Application metadata and navigation JSON

## Data sources

- **Owner-supplied reference application:** `C:\Users\BMW\Videos\Thai League (T1-T5)\thai_league_05.html`  
  SHA-256: `c7cf3e61c59760463d085006676e010a84262b7776699490c5b9e81c3322cd3e`
- Football Association of Thailand: <https://www.fathailand.org/>
- Thai League: <https://www.thaileague.co.th/>
- Wikipedia reference pages for 2025–26 Thai League 1/2/3, FA Cup, League Cup
- Department of Physical Education: <https://www.dpe.go.th/>
- Channel 7HD: <https://www.ch7.com/>
- Owner-approved naming/category instructions

The imported standings, results, fixtures, cup ties, scorers and news are marked **unverified**. Source links are shown in the `Sources & data notes` panel on each competition page.

## Data handling rules

- Original imported values are preserved.
- T1 arithmetic passes validation; T2 has two `MP ≠ W+D+L` rows; all six T3 regional groups fail arithmetic validation and are displayed with warnings.
- Imported result/fixture/scorer records that cannot be linked to the relevant season teams are retained in the snapshot but excluded from displayed tables; the UI reports the unresolved count.
- A small documented alias map handles `Chainat Hornbill`, `Pattaya Utd`, and `Nakhon Si Utd`; these aliases remain marked unverified.
- School and youth team lists are curated from public draw/results reporting; they are secondary sources unless an official federation/DPE page was captured.
- Teams, fixtures, results, scorers, and champions are never fabricated. Categories without an accessible public list show an honest empty state with the reason.
- T4/T5 legacy data is preserved separately and is not active.

## Project files

```text
src/                 Axum routes, Askama page views, domain models, preview/DB loaders
templates/           Askama layout, page, search, partials
styles/app.css       Tailwind v4 source + custom visual system
static/              Built CSS, app.js, vendored HTMX, favicon
data/seed.json       Project-owned preview snapshot
data/legacy_t4_t5.json
migrations/          SQLx PostgreSQL schema
src/bin/import_seed.rs
tools/               Data extractors, curated-team builder, browser verification (development only)
tests/               Domain and HTTP tests
```

## Remaining limitations

- Imported football data is a March 2026 snapshot and is not independently verified.
- School Type A/B and youth cup lists are populated from public secondary reporting. Type A U14 uses the latest complete 2024/2567 list. General school categories, the College Cup, and the three FAT youth leagues have no complete 2026 public list and show honest empty states.
- Cup schedules contain imported date/note conflicts and are shown with warnings.
- PostgreSQL mode requires a project-owned database and has not been runtime-tested.
- No push, Render deployment, or Neon provisioning has been performed.

## Visual direction

The UI follows the owner-approved direction: light background, white content panels, thin borders, restrained teal accent, compact navigation, clear typography, small status badges, and an optional dark theme. It is an independent Thailand Football implementation and contains no Academic Life content, branding, records, or code.
