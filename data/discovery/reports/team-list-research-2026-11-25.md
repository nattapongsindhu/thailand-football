# Team List Research — 2026-11-25

Scope: visible team names in the Teams tab for the 17 owner-approved competitions.

Method:

- Public web sources only. No private groups, logins, or paywalled pages were used.
- Thai and English search terms were tried; the Wayback Machine was used for pages that returned HTTP 403.
- National cup participants were parsed from the English Wikipedia season articles saved in
  `data/discovery/sources/cup_participants_2025_26.json`.
- A team is only added when a public source names it. Categories without a complete list keep an honest empty state.
- `verified_secondary` means a secondary public source (news/Wikipedia) supports the participation. It does not mean official confirmation.
- `discovered_unverified` means the name appeared in public reporting but the source is weaker (social post) or no separate club article could be linked.

## Roster fallback status (latest verified roster policy)

The participation `season_id` remains the current UI season. The metadata below records which season the visible roster actually came from. Fallback rosters are never joined to standings, fixtures, results, or statistics.

Source checked date for every roster: **2026-11-25**.

| Competition | UI season | Displayed season | Source season | Teams | Roster status | Source URL |
|---|---|---|---:|---:|---|---|
| Thai Youth League 1 (U18) | 2026 | 2026 | 2026 | 0 | unavailable | — |
| Thai Youth League 2 (U16) | 2026 | 2026 | 2026 | 0 | unavailable | — |
| Thai Youth League 3 (U14) | 2026 | 2026 | 2026 | 0 | unavailable | — |
| Thai Youth FA Cup (U17) | 2026 | 2026 | 2026 | 12 | partial | https://siamsport.co.th/football-thailand/th-other/107933 |
| Thai Youth League Cup (U18) | 2026 | 2026 | 2026 | 8 | partial | https://thethaiger.com/th/news/1634775 |
| Thai Youth Champion Cup (U18) | 2026 | **2025** | **2025** | 4 | latest_verified | https://matichon.co.th/sport/news_5456292 |
| Thai School League — Type A U18 | 2026-u18 | 2026 | 2026 | 16 | current | https://www.thaipost.net/sport-news/991753 |
| Thai School League — Type A U16 | 2026-u16 | 2026 | 2026 | 16 | current | https://www.thaipost.net/sport-news/991753 |
| Thai School League — Type A U14 | 2026-u14 | **2024** | **2024** | 16 | latest_verified | https://www.siamsport.co.th/football-thailand/th-other/60160/ |
| Thai School League — Type B U18 | 2026-u18 | 2026 | 2026 | 16 | current | https://www.matichon.co.th/sport/footballlocal/news_5836043 |
| Thai School League — Type B U16 | 2026-u16 | 2026 | 2026 | 16 | partial | https://www.facebook.com/footballromcom/posts/1767809752018325/ |
| Thai School League — Type B U14 | 2026-u14 | 2026 | 2026 | 0 | unavailable | — |
| Thai School League — General U18 | 2026-u18 | 2026 | 2026 | 0 | unavailable | — |
| Thai School League — General U16 | 2026-u16 | 2026 | 2026 | 0 | unavailable | — |
| Thai School League — General U14 | 2026-u14 | 2026 | 2026 | 0 | unavailable | — |
| Thai School League — General U12 | 2026-u12 | 2026 | 2026 | 0 | unavailable | — |
| Thai College Cup (U19) | 2026 | 2026 | 2026 | 0 | unavailable | — |
| Thai University Cup (U23) | 2026 | 2026 | 2026 | 7 | partial | https://edu-today.com/btu-215 |

Notes:

- `latest_verified` means a complete previous-season roster is shown because the current roster was not published or verified.
- `partial` means only a documented subset is shown (for example a final round or confirmed knockout participants).
- `unavailable` means no reliable current or previous roster exists; the Teams tab keeps its honest empty state.
- Search result season labels use the displayed season, so a 2025 roster is not shown as 2026.

## Populated lists

| Competition | Season / age | Teams | Verification | Source |
|---|---|---:|---|---|
| Thai League 1 | 2025/26 | 16 | verified secondary | English Wikipedia, 2025–26 Thai League 1 |
| Thai League 2 | 2025/26 | 18 | verified secondary | English Wikipedia, 2025–26 Thai League 2 |
| Thai League 3 | 2025/26 | 35 | discovered unverified | Owner-supplied standings snapshot |
| Thai FA Cup | 2025/26 | 95 | 77 secondary + 18 unverified | English Wikipedia, 2025–26 Thai FA Cup |
| Thai League Cup | 2025/26 | 80 | verified secondary | English Wikipedia, 2025–26 Thai League Cup |
| Thai Champion Cup | 2026 | 2 | verified secondary | English Wikipedia, 2026–27 Buriram United season (Buriram v Port) |
| Thai School League — Type A | 2026 U18 | 16 | verified secondary | Thai Post, 6 May 2026, DPE Deli News Cup 2026 |
| Thai School League — Type A | 2026 U16 | 16 | verified secondary | Thai Post, 6 May 2026, DPE Deli News Cup 2026 |
| Thai School League — Type A | 2026 U14 | 16 | verified secondary | Siamsport, 23 Aug 2024 (latest complete list found, 2024/2567) |
| Thai School League — Type B | 2026 U18 | 16 | verified secondary | Matichon, 6 Aug 2026, DPE WOW Cup 2026 |
| Thai School League — Type B | 2026 U16 | 16 | discovered unverified | Public youth-football Facebook preview, 2026 (official DPE draw not captured) |
| Thai Youth FA Cup (U17) | 2026 | 12 | verified secondary | Siamsport, 14 Aug 2026, Coke Cup U17 national finalists |
| Thai Youth League Cup (U18) | 2026 | 8 | verified secondary | The Thaiger, 24 Nov 2026, 7HD Champion Cup final eight |
| Thai Youth Champion Cup (U18) | 2026 | 4 | verified secondary | Matichon, Nov 2025, 31st Jaturamitr Samakkhi |
| Thai University Cup (U23) | 2026 | 7 | verified secondary | Education Today, 2026 Chang U-Champion Cup knockout reporting (partial field) |

## Empty categories and reasons

### Thai School League — General (U18 / U16 / U14 / U12)

- The General category is an open-entry knockout. Public reporting for 2025/2568 states 413 teams entered.
- A complete entry list was published only as a social-media image/PDF; no machine-readable public list was accessible.
- The 2026/2569 General competition was reported as unfunded and not confirmed at capture time (Dailynews, 6 Oct 2026).
- Search terms used: `ประเภททั่วไป 2568 ฟุตบอลกรมพลศึกษา 413 ทีม`, `18 ปี ทั่วไป 2569`, `16 ปี ทั่วไป 2569`, `14 ปี ทั่วไป 2569`, `12 ปี ทั่วไป 2568`.
- Decision: do not present four promoted teams as the full General field. Empty state explains the limitation.

### Thai School League — Type B U14

- No 2026/2569 Type B U14 public team list was located.
- The latest published final was Ratchawinit Bang Kaeo vs Mathayom Taksin Rayong (2025/2568); a full 16-team list for that edition was not found in accessible text.
- Search terms used: `14 ปี ข 2569 16 ทีม`, `14 ปี ข DPE WOW`, `14 ปี ข 2568 รายชื่อ`.

### Thai College Cup (U19)

- The 15th national vocational games (Aug 2026) included U19 football, but public reporting names individual colleges in match reports only.
- No complete 16-team (or final-round) list was located.
- Search terms used: `ฟุตบอลอาชีวะระดับประเทศ 2569`, `อาชีวะเกมส์ ครั้งที่ 15 ฟุตบอล 19 ปี`, `ฟุตบอลอาชีวะ อายุไม่เกิน 19 ปี ชิงแชมป์ประเทศไทย`.

### Thai Youth League 1 (U18), 2 (U16), 3 (U14)

- The FA Thailand competition calendar for 2025/26 and 2026/27 does not list an FA Thailand Youth League.
- The last official FA Thailand Youth League edition found was 2022/23.
- ONCB Thai Youth League and TIFAA Youth League are different competitions and were not substituted.
- Search terms used: `FA Thailand Youth League 2025/26`, `site:fathailand.org Youth League 2569`, `ฟุตบอลเยาวชน สมาคมฯ ลีก 1`.
- Checked pages: `fathailand.org/match-results` season 32 (2025/26) and season 33 (2026/27).

## Caveats

- Cup participant lists include 18 FA Cup entrants whose names have no separate Wikipedia club article; they are marked unverified rather than linked to an existing club.
- Type A U14 uses the 2024/2567 edition because the 2025/2568 and 2026/2569 full lists were not published in accessible text.
- The 31st Jaturamitr Samakkhi was held in November 2025. No 2026 football edition was held, so the four schools are labelled as the 2025 edition.
- The University Cup list is partial: only knockout-stage participants confirmed in public reporting are shown; the full 16-team field was not published as text.
- Team identity is never merged by name alone. Similar names (for example Phrae / Phrae Utd / Phrae TS, or Chiangmai / Chiang Mai FC) remain separate IDs.

## Reproduction

```powershell
node tools/build_curated_teams.mjs
cargo run --bin update-football-data
```

Offline metadata-only run:

```powershell
$env:TF_DISCOVERY_NO_FETCH = '1'
cargo run --bin update-football-data
```
