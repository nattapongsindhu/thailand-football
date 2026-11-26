# PROJECT STATUS — Thailand Football

**Status:** local implementation complete; local preview server running.  
**Phase:** local application build and verification.  
**Production deployment / GitHub push:** not authorized and not performed.

## Current local server

- URL: `http://127.0.0.1:8080`
- Process ID: **32912** (also in `logs/server.pid`)
- Bind address: `127.0.0.1`
- Data mode: **Local preview** (read-only JSON snapshot; no `DATABASE_URL` configured)
- Logs: `logs/server.out.log`, `logs/server.err.log`
- Stop: `Stop-Process -Id 32912`

## Implemented

- Rust stable edition 2024 monolith: Axum + Tokio + Askama + SQLx
- Tailwind CSS v4 production build
- Owner-approved organizer groups: National League, Youth League (including youth cups), and School League (including vocational and university cups)
- Expandable age-group navigation shared by School League and Youth League: Thai Youth League 1/2/3 are parents with U18/U16/U14 child seasons; legacy single-age records map onto their matching age group.
- Thai League 1/2/3 standings; T3 regional group navigation
- Results and fixtures where linked records exist
- Team statistics derived from standings
- Top scorers with resolved team links
- FA Cup, League Cup, Champion Cup rounds/ties
- Competition-specific news with no senior-data fallback
- Stable team-ID search with competition/group/season context and row highlighting
- Stable team-ID favorites in localStorage
- Light/dark theme in localStorage
- Responsive sidebar, tables, match cards, and cup views
- Empty states for youth categories with no verified records
- Data-aware default tab: competition pages open on Teams when standings or cup rounds have no records, so school and youth team lists are visible immediately
- Latest-verified roster fallback: current roster first, then the latest complete previous-season roster, with displayed/source season metadata, roster-status badges, and a notice. Fallback teams never enter standings, fixtures, results, or statistics.
- Read-only local preview mode
- PostgreSQL schema, SQLx loader, and controlled import binary
- `/healthz`
- Security headers and a self-only CSP

## Data status

- Source snapshot: `C:\Users\BMW\Videos\Thai League (T1-T5)\thai_league_05.html`
- SHA-256: `c7cf3e61c59760463d085006676e010a84262b7776699490c5b9e81c3322cd3e`
- Active competitions: 17
- Active team/standing rows: 69
- Search entries: 365
- News articles: 13
- Team-first registry: **196 teams / 369 participations**
  - 318 verified secondary
  - 51 discovered/unverified
  - 0 verified official
- Roster metadata: **24 Youth/School roster records**
  - 3 current
  - 2 latest_verified (Type A U14 displayed/source 2024; Youth Champion Cup displayed/source 2025)
  - 5 partial (Youth FA Cup, Youth League Cup, Type B U16, University Cup, College Cup)
  - 14 unavailable (Youth League 1/2/3 × U18/U16/U14, Type B U14, General U18/U16/U14/U12)
- Teams tab coverage:
  - Thai League 1: 16
  - Thai League 2: 18
  - Thai League 3: 35
  - Thai FA Cup: 95 (all verified secondary; complete 2025-26 results)
  - Thai League Cup: 80 (all verified secondary; complete 2025-26 results)
  - Thai Champion Cup: 2
  - School League Type A: 48 (U18 16, U16 16, U14 16)
  - School League Type B: 32 (U18 16 verified secondary, U16 16 unverified)
  - School League General: 0 (honest empty state)
  - Thai Youth FA Cup (U17): 12
  - Thai Youth League Cup (U18): 8
  - Thai Youth Champion Cup (U18): 4
  - Thai University Cup (U23): 11 (confirmed knockout-stage participants; partial field)
  - Thai College Cup (U19): 8 (corrected 2026 quarter-finalists; partial field)
  - Thai Youth League 1/2/3: 0 each age group (U18/U16/U14 honest empty states)
- Cup results: Thai FA Cup 7 rounds / 94 ties (champion Buriram United, runner-up PT Prachuap); Thai League Cup 8 rounds / 79 ties (champion Port FC, runner-up BG Pathum United). Penalty scores and extra-time status are stored separately; awarded results are labelled.
- Youth/university cup records: Coke Cup U17 quarter-finals, semi-finals and final (champion Assumption United); 7HD Champion Cup 2026 quarter-final draw (matches 30 Nov 2026, status Upcoming); 31st Jaturamitr Samakkhi 2025 full matchdays and final (champion Debsirin School); Chang U-Champion Cup 2026 quarter-finals, semi-finals and final (champion Thailand National Sports University).
- Discovery candidates: 379 in inbox
- Discovery news metadata: 42 items
- Source registry: 17 competitions; 8 public sources fetched, 0 failed
- Curated source file: `data/discovery/curated_verified_teams.json` (330 participations before merge)
- Research log: `data/discovery/reports/team-list-research-2026-11-25.md`
- T4/T5 legacy records: preserved in `data/legacy_t4_t5.json` and inactive
- Imported standings and legacy values remain marked unverified; FA Cup and League Cup entries/results are `verified_secondary`.

### Known data limitations

- T1 passes arithmetic validation.
- T2 has two rows with `MP ≠ W+D+L`.
- All six T3 regional groups fail arithmetic validation; warnings are displayed.
- Some results/fixtures/scorers reference teams absent from the relevant standing table; they are preserved but not displayed, with an unresolved count shown.
- Documented unverified aliases: `Chainat Hornbill`, `Pattaya Utd`, `Nakhon Si Utd`.
- FA Cup and League Cup warnings were removed because the complete 2025-26 results replace the stale schedule notes.
- School and youth lists are curated from public secondary reporting, not official federation/DPE confirmation.
- Type A U14 uses the latest complete 2024/2567 list; the 2025/2568 and 2026/2569 full lists were not published in accessible text.
- The 2026 Jaturamitr Samakkhi football edition was not held; the four schools are labelled as the 31st (2025) edition.
- The University Cup list is partial (11 confirmed knockout participants); the full 16-team field was not published as text. Quarter-final scores for three ties were not published, but the advancing teams were confirmed.
- The College Cup list is partial: the corrected 2026 bracket has eight quarter-finalists across three rounds. Two quarter-final scores are unavailable and their winners are inferred from the semi-final lineup; Chonburi's 5–4 semi-final shootout win is stored separately from the 1–1 score. The Thawee SC report's Yasothon final claim is rejected in favour of the owner-supplied result image (Chonburi 0–2 Thawee SC).
- Honest empty states remain for School League General (all ages), Type B U14, and all nine Youth League parent/age combinations because no complete 2026 public list was located. The exact search terms and reasons are recorded in the research log.
- Fallback participation season IDs remain the current UI season. The displayed/source season metadata, Teams-tab badge/notice, and search-result labels show the actual roster season, so a 2025 roster is never presented as 2026.
- Partial lists (Youth FA Cup, Youth League Cup, Type B U16, University Cup, College Cup) are labelled `Partial list`; unavailable categories keep their honest empty state.

## Verification

- `cargo fmt --all -- --check`: passed
- `cargo check --all-targets`: passed
- `cargo clippy --all-targets -- -D warnings`: passed
- `cargo test --all-targets`: 23 tests passed (15 domain + 6 HTTP + 2 discovery)
- Tailwind CSS production build: passed
- HTTP Teams-tab checks: 24/24 passed (all 17 competitions, school age groups, empty-state messages)
- Browser checks: 100 checks passed, 0 failures, no browser console errors; includes FA Cup/League Cup/College Cup rounds, Coke Cup quarter-finals, 7HD draw, Jaturamitr and University Cup quarter-finals, Youth League navigation, fallback badges, and School/National League regressions
- Fallback policy tests confirm latest_verified/partial/unavailable labels and that fallback teams do not populate standings, fixtures, results, stats, or scorers
- Team discovery: 196 teams / 369 participations; 318 verified secondary, 51 discovered unverified
- News metadata: 42 items; 379 inbox candidates; 8 public sources fetched, 0 failed
- Local Windows-gnu builds used `C:\Users\BMW\mingw64\bin` on `PATH`; the rustup self-contained linker on this machine otherwise fails with `nanosleep64` from `aws-lc-sys`.
- Browser verification covered desktop/mobile layout, favorites persistence, theme persistence, HTMX search, search-to-group navigation, school league distinction and Thai subtitles, complete FA Cup/League Cup rounds, U23 empty state, and mobile sidebar
- Inactive legacy route 404 is covered by the HTTP test suite
- Browser screenshots saved to `logs/` during verification
- College Cup bracket regression test covers the eight quarter-finalists, three rounds, unavailable scores with inferred winners, the separate 5–4 penalty result, the 0–2 final, and the absence of a Yasothon finalist.

## PostgreSQL mode

Verified locally against a temporary PostgreSQL 17 cluster on `127.0.0.1:55432` (trust auth, isolated from the machine's normal PostgreSQL service):

- Migration `0001_init.sql`: applied.
- Migration `0002_team_discovery.sql`: applied after fixing the invalid `participations.competition_id` index to `(season_key, group_key)`.
- `import_seed`: completed.
- `import-team-registry`: completed after creating missing `season_groups` rows and fixing `display_name`.
- `app_meta.mode`: set to `postgresql`.
- Release binary bound to `0.0.0.0:8091` and reported `mode: postgresql`, 17 competitions, 357 search entries.
- National League Teams/Standings, Youth Teams, School Teams, `latest_verified` badges, empty states, and no fallback leakage were verified in PostgreSQL mode.

College Cup correction re-verified locally against a temporary PostgreSQL 17 cluster on `127.0.0.1:55433`:

- Migration `0003_college_cup_tie_metadata.sql`: applied.
- `import_seed`: completed; College Cup ties retained the unavailable-score flags, inferred winners, and separate penalty scores.
- `import-team-registry`: completed with 193 teams / 365 participations.
- The College Cup Teams tab showed 8 rows and the Rounds tab showed 3 rounds, the unavailable-score labels, the 4–5 penalty line, and a final without Yasothon.

2025-26 FA Cup and League Cup import re-verified locally against a temporary PostgreSQL 17 cluster on `127.0.0.1:55434`:

- Migration `0004_cup_tie_extra_time.sql`: applied.
- `import_seed`: completed; 94 FA Cup ties (10 extra-time, 14 shootouts) and 79 League Cup ties (19 extra-time, 11 shootouts) were stored with separate score/penalty/extra-time columns.
- `import-team-registry`: completed with 193 teams / 365 participations.
- PostgreSQL-rendered pages showed 7/94 and 8/79 ties with champions, penalties, and AET status; College Cup and Youth League pages remained intact.

No project-owned Neon database was available for this run. The production Neon connection string is still required in Render's `DATABASE_URL`.

## Deployment readiness

- `render.yaml` defines one free Rust web service with `cargo build --release --locked`, `./target/release/thailand-football`, and `/healthz`.
- The application reads `PORT` and binds `0.0.0.0:$PORT` when set; local mode still uses `127.0.0.1:8080-8089`.
- `DEPLOYMENT.md` documents the GitHub → Neon → Render runbook.

### Current deployment blockers

- Neon CLI is not signed in (`neon projects list` returns `Not signed in`), and no `NEON_API_KEY` is present in the environment.
- Render CLI is not installed and no `RENDER_API_KEY`/session is available; the browse extension relay is also offline.
- These two account-authentication blockers prevent creating the Neon database and the Render service from this session.

## Not performed

- GitHub push
- Render deployment
- Neon provisioning
- Paid services
- Production DNS/domain changes
- Scraping
- Any change to another project

## Next owner/architect decisions

1. Provide or approve a project-owned PostgreSQL/Neon database for DB-mode verification.
2. Confirm whether the imported March 2026 snapshot remains the baseline.
3. Supply verified corrections for T2/T3/results/scorers, or approve keeping warnings.
4. Review the curated school/youth lists against official DPE/FAT documents and supply official confirmations where available.
5. Decide public/private access (private access would require authentication).
6. Confirm media/logo rights.
7. Approve GitHub push and Render deployment when ready.
