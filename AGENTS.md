# AGENTS.md — Thailand Football

## Project ownership

- **Owner:** Rogers
- **Project manager / system architect:** Codex
- **Implementation developer:** OpenCode Go
- **Repository target:** `https://github.com/nattapongsindhu/thailand-football` (private; no push is authorized by this phase)

This is an independent project. Do not read, copy, import, or modify source code, databases, credentials, environment files, assets, or configuration from Academic Life or any other project.

## Approved technology

- Rust stable, edition 2024
- Axum
- Tokio
- Askama
- SQLx
- PostgreSQL / Neon PostgreSQL (future)
- HTML
- Tailwind CSS v4
- Minimal vanilla JavaScript
- HTMX only where useful
- GitHub
- Render Free (future)

Use one Rust monolith. Do not introduce React, Next.js, Vue, Angular, PHP, a Python backend, a Node.js backend, microservices, Redis, GraphQL, Docker, Kubernetes, Supabase, Vercel, SQLite, or paid services.

Development utilities (Tailwind CLI, Playwright, one-off extraction/import tools) are allowed but must not become production runtime dependencies.

## Latest owner-approved sidebar structure

### National League

- Thai League 1
- Thai League 2
- Thai League 3
- Thai FA Cup
- Thai League Cup
- **Thai Champion Cup** (exact display name)

### Youth League (FAT & SAT)

- Thai Youth League 1 (U18)
  - Thai label: ฟุตบอลเยาวชน สมาคมฯ ลีก 1
- Thai Youth League 2 (U16)
  - Thai label: ฟุตบอลเยาวชน สมาคมฯ ลีก 2
- Thai Youth League 3 (U14)
  - Thai label: ฟุตบอลเยาวชน สมาคมฯ ลีก 3
- Thai Youth FA Cup (U17)
  - Thai label: ฟุตบอลเยาวชน โค้กคัพ Coke Cup
  - Alias / sponsor name: Coke Cup
- Thai Youth League Cup (U18)
  - Thai label: ฟุตบอลเยาวชน แชมป์กีฬา 7HD
  - Alias / sponsor name: 7HD Champion Cup
- Thai Youth Champion Cup (U18)
  - Thai label: ฟุตบอลเยาวชน จตุรมิตรสามัคคี

### School League (DPE)

- Thai School League — Type A
  - U18
  - U16
  - U14
  - Thai label: ฟุตบอลนักเรียน กรมพลฯ ประเภท ก
- Thai School League — Type B
  - U18
  - U16
  - U14
  - Thai label: ฟุตบอลนักเรียน กรมพลฯ ประเภท ข
- Thai School League — General
  - U18
  - U16
  - U14
  - U12
  - Thai label: ฟุตบอลนักเรียน กรมพลฯ ประเภท ทั่วไป
- Thai College Cup (U19)
  - Thai label: ฟุตบอลอาชีวะระดับประเทศ
- Thai University Cup (U23)
  - Thai label: ฟุตบอลมหาวิทยาลัยระดับประเทศ
- Thai School Super Cup (U23)
  - Thai label: ฟุตบอลประเพณี มธ-จุฬาฯ

### Naming rules

- U19 is included. The earlier exclusion of U19 is superseded.
- University football is U23. U20 is superseded as an active university label.
- School competitions use **Type A**, **Type B**, and **General**; do not use School League 1/2/3, because numbers imply hierarchy or promotion.
- Type A, Type B, and General are separate competition identities.
- Age groups U18/U16/U14 (and U12 for General) are season-level age dimensions. They are selectable beneath each school competition in the sidebar.
- Do not restore the old ก./ข./ค./ง. subcategory sidebar from the previous structure.
- Navigation grouping must remain separate from permanent competition identity.
- Thai organizer descriptions and Thai item labels are displayed below the English names where supplied.
- Sponsor names are aliases, not primary names. Coke Cup and 7HD Champion Cup must remain separate from the standard competition names.
- Thai Youth FA Cup (U17) and Thai Youth League Cup (U18) belong to the Youth League organizer group.
- Thai College Cup (U19), Thai University Cup (U23), and Thai School Super Cup (U23) belong to the School League organizer group; do not restore separate Sponsor or Education Cups sections.
- Owner-defined categories are not evidence of official eligibility or a connected promotion/relegation pyramid.
- Thai League 4 and Thai League 5 are preserved as inactive legacy data. They must not appear as active sidebar competitions.
- Thai School Super Cup (U23) is the owner-approved traditional Thammasat–Chulalongkorn football match and is active at the bottom of the School League group. Do not generate data for it beyond the sourced latest edition.

### Data structure

Organizer Group → Competition → Season → Age Group / Type → Teams → Fixtures / Results

### Team discovery

- Source registry: `data/source_registry.json`
- Team-first registry: `data/team_registry.json`
- Curated verified teams: `data/discovery/curated_verified_teams.json`
- News metadata: `data/discovery/news_metadata.json`
- Discovery inbox: `data/discovery/inbox/`
- Reports: `data/discovery/reports/latest.json` and `latest.md`
- Local update command:

```powershell
cargo run --bin update-football-data
```

Use `TF_DISCOVERY_NO_FETCH=1` for an offline/discovery-metadata-only run.

Verification statuses: `verified_official`, `verified_secondary`, `discovered_unverified`, `conflicting`, `rejected`.

Rules:

- Teams are permanent identities; participations are separate and carry competition, season, age group, type, source, and verification.
- The same name in two competitions is not automatically the same team.
- Do not merge reserve, school, club, or age-group teams by name alone.
- Automatic inbox candidates must not be promoted to the team registry without a source-based curation step.
- News metadata stores headlines and short summaries only; never copy full articles.
- The local update command must not require PostgreSQL, GitHub, Render, or Neon.

## Data rules

- Preserve imported values. Do not silently correct arithmetic, dates, aliases, or competition names.
- Imported standings are marked unverified unless independently checked.
- Repeated names do not prove duplicate teams. Do not merge teams by name alone.
- Search and favorites use stable team IDs.
- Empty youth categories must show an honest empty state. Do not fabricate teams, fixtures, results, scorers, or champions.
- Record source URLs and applicable seasons for verified additions.

## Commands

```powershell
npm run css:build
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo build
.\target\debug\thailand-football.exe
```

Local URL: `http://127.0.0.1:8080` (the binary tries ports 8080–8089 if occupied).

## Approval gates

Do not perform any of the following without explicit owner/architect approval:

- Push to GitHub
- Create or modify a Render service
- Create or modify a Neon/PostgreSQL cloud resource
- Deploy to production
- Add paid services
- Modify another project

Local implementation, local schema/import files, local builds, local checks, and the local preview server are authorized.
