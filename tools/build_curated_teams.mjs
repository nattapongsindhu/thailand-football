/**
 * Build curated team participations from public sources.
 *
 * Cup participants come from the English Wikipedia season articles saved in
 * data/discovery/sources/cup_participants_2025_26.json. School, youth and
 * university lists come from the public news reports cited inline below.
 *
 * Run: node tools/build_curated_teams.mjs
 */
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(import.meta.dirname, "..");
const CURATED = path.join(ROOT, "data", "discovery", "curated_verified_teams.json");
const CUP_SOURCE = path.join(
  ROOT,
  "data",
  "discovery",
  "sources",
  "cup_participants_2025_26.json",
);

const NOW = "2026-11-25";
const COLLEGE_CUP_SOURCE_URL =
  "https://www.facebook.com/100064329743962/posts/1503699835117660/";
const curated = JSON.parse(fs.readFileSync(CURATED, "utf8"));

// The script owns these competitions; clear their participations so a re-run
// never leaves a stale list behind.
const managedCompetitions = new Set([
  "thai-fa-cup",
  "thai-league-cup",
  "thai-champion-cup",
  "thai-youth-league-u18",
  "thai-youth-league-u16",
  "thai-youth-league-u14",
  "thai-youth-fa-cup-u17",
  "thai-youth-league-cup-u18",
  "thai-youth-champion-cup-u18",
  "thai-school-league-type-a",
  "thai-school-league-type-b",
  "thai-school-league-general",
  "thai-college-cup-u19",
  "thai-university-cup-u23",
]);
curated.participations = curated.participations.filter(
  (participation) => !managedCompetitions.has(participation.competition_id),
);
curated.rosters = (curated.rosters || []).filter(
  (roster) => !managedCompetitions.has(roster.competition_id),
);

// Roster fallback policy for youth and school competitions. The participation
// season_id stays on the current UI season; these records explain which season
// the visible roster actually came from.
const rosterDefinitions = {
  "thai-youth-fa-cup-u17|2026": {
    age_group: "U17",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "partial",
    source_url: "https://siamsport.co.th/football-thailand/th-other/107933",
    note: "National final round only: 12 of 192 entrants.",
  },
  "thai-youth-league-cup-u18|2026": {
    age_group: "U18",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "partial",
    source_url: "https://thethaiger.com/th/news/1634775",
    note: "National final eight only.",
  },
  "thai-youth-champion-cup-u18|2026": {
    age_group: "U18",
    displayed_season: "2025",
    source_season: "2025",
    roster_status: "latest_verified",
    source_url: "https://matichon.co.th/sport/news_5456292",
    note: "31st Jaturamitr Samakkhi (2025); no 2026 football edition was held.",
  },
  "thai-school-league-type-a|2026-u18": {
    age_group: "U18",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "current",
    source_url: "https://www.thaipost.net/sport-news/991753",
    note: "2026 (2569) Type A U18 list.",
  },
  "thai-school-league-type-a|2026-u16": {
    age_group: "U16",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "current",
    source_url: "https://www.thaipost.net/sport-news/991753",
    note: "2026 (2569) Type A U16 list.",
  },
  "thai-school-league-type-a|2026-u14": {
    age_group: "U14",
    displayed_season: "2024",
    source_season: "2024",
    roster_status: "latest_verified",
    source_url: "https://www.siamsport.co.th/football-thailand/th-other/60160/",
    note: "Latest complete public Type A U14 list (2024/2567).",
  },
  "thai-school-league-type-b|2026-u18": {
    age_group: "U18",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "current",
    source_url: "https://www.matichon.co.th/sport/footballlocal/news_5836043",
    note: "2026 (2569) Type B U18 list.",
  },
  "thai-school-league-type-b|2026-u16": {
    age_group: "U16",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "partial",
    source_url: "https://www.facebook.com/footballromcom/posts/1767809752018325/",
    note: "Public preview of the 2026 Type B U16 list; official DPE draw not captured.",
  },
  "thai-school-league-type-b|2026-u14": {
    age_group: "U14",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "unavailable",
    source_url: null,
    note: "No 2026/2569 Type B U14 public list located.",
  },
  "thai-school-league-general|2026-u18": {
    age_group: "U18",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "unavailable",
    source_url: null,
    note: "Open-entry General category; no complete 2026 public list.",
  },
  "thai-school-league-general|2026-u16": {
    age_group: "U16",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "unavailable",
    source_url: null,
    note: "Open-entry General category; no complete 2026 public list.",
  },
  "thai-school-league-general|2026-u14": {
    age_group: "U14",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "unavailable",
    source_url: null,
    note: "Open-entry General category; no complete 2026 public list.",
  },
  "thai-school-league-general|2026-u12": {
    age_group: "U12",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "unavailable",
    source_url: null,
    note: "Open-entry General category; no complete 2026 public list.",
  },
  "thai-college-cup-u19|2026": {
    age_group: "U19",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "partial",
    source_url: COLLEGE_CUP_SOURCE_URL,
    note: "Quarter-final participants in the corrected 2026 bracket; the full entry list is unavailable. An alternate final claim naming Yasothon is rejected.",
  },
  "thai-university-cup-u23|2026": {
    age_group: "U23",
    displayed_season: "2026",
    source_season: "2026",
    roster_status: "partial",
    source_url: "https://edu-today.com/btu-215",
    note: "Confirmed knockout-stage participants only (8 of 16).",
  },
};

// Each Youth League parent owns three age-group seasons (U18, U16, U14).
// The legacy single-age records map onto the matching age group.
for (const competitionId of [
  "thai-youth-league-u18",
  "thai-youth-league-u16",
  "thai-youth-league-u14",
]) {
  for (const age of ["U18", "U16", "U14"]) {
    rosterDefinitions[`${competitionId}|2026-${age.toLowerCase()}`] = {
      age_group: age,
      displayed_season: "2026",
      source_season: "2026",
      roster_status: "unavailable",
      source_url: null,
      note: "No 2025/26 or 2026/27 FA Thailand Youth League team list is published.",
    };
  }
}

const teams = new Map(curated.teams.map((team) => [team.team_id, team]));
const participations = [...curated.participations];
const partKeys = new Set(participations.map(participationKey));

function participationKey(participation) {
  return [
    participation.team_id,
    participation.competition_id,
    participation.season_id,
    participation.group_id || "overall",
  ].join("|");
}

function addTeam(team) {
  if (teams.has(team.team_id)) return;
  teams.set(team.team_id, team);
}

function addParticipation(participation) {
  const key = participationKey(participation);
  if (partKeys.has(key)) return;
  participations.push(participation);
  partKeys.add(key);
}

function slug(value) {
  return value
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/&/g, " and ")
    .replace(/\bF\.?C\.?\b/gi, "")
    .replace(/\((thailand|2023)\)/gi, "")
    .replace(/[^a-zA-Z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();
}

function newTeam(id, displayName, thaiName, organizationType, notes) {
  return {
    team_id: id,
    display_name: displayName,
    thai_name: thaiName || null,
    english_name: displayName,
    aliases: [displayName, ...(thaiName ? [thaiName] : [])],
    organization_type: organizationType,
    province: null,
    verification_status: "verified_secondary",
    first_seen_at: NOW,
    last_seen_at: NOW,
    notes,
  };
}

function newParticipation({
  teamId,
  competitionId,
  seasonId,
  ageGroup,
  divisionType,
  source,
  verification = "verified_secondary",
  notes,
}) {
  const roster = rosterDefinitions[`${competitionId}|${seasonId}`];
  return {
    participation_id: `part-${competitionId}-${seasonId}-${teamId}`,
    team_id: teamId,
    competition_id: competitionId,
    season_id: seasonId,
    age_group: ageGroup || null,
    group_id: "overall",
    division_type: divisionType,
    source_url: source.url,
    source_title: source.title,
    source_publisher: source.publisher,
    source_date: source.date,
    verification_status: verification,
    first_seen_at: NOW,
    last_seen_at: NOW,
    notes,
    displayed_season: roster?.displayed_season ?? null,
    source_season: roster?.source_season ?? null,
    roster_status: roster?.roster_status ?? null,
    source_checked_at: roster ? NOW : null,
  };
}

/* ------------------------------------------------------------------ */
/* National cups: Thai FA Cup and Thai League Cup                      */
/* ------------------------------------------------------------------ */

const cupSources = JSON.parse(fs.readFileSync(CUP_SOURCE, "utf8"));

// Existing stable identities for clubs already in the registry.
const existingClubIds = {
  "Ayutthaya United F.C.": "team-ayutthaya-utd",
  "Bangkok F.C.": "team-bangkok-fc",
  "Bangkok United F.C.": "team-bangkok-utd",
  "Bankhai United F.C.": "team-ban-khai-united",
  "BG Pathum United F.C.": "team-pathum-united",
  "Burapha United F.C.": "team-burapha-united",
  "Buriram United F.C.": "team-buriram-united",
  "Chainat Hornbill F.C.": "team-chainat",
  "Chanthaburi F.C.": "team-chanthaburi",
  "Chiangmai United F.C.": "team-chiang-mai-fc",
  "Chiangrai United F.C.": "team-chiangrai-utd",
  "Chonburi F.C.": "team-chonburi",
  "Customs United F.C.": "team-customs-united",
  "Futera United F.C.": "team-futera-united",
  "Hua Hin City F.C.": "team-hua-hin-city",
  "Kanchanaburi Power F.C.": "team-kanchanaburi",
  "Kasem Bundit University F.C.": "team-kasem-bundit-u",
  "Kasetsart F.C.": "team-kasetsart",
  "Khon Kaen United F.C.": "team-khon-kaen-united",
  "Krabi F.C.": "team-krabi-fc",
  "Lamphun Warriors F.C.": "team-lamphun-warrior",
  "Lopburi City F.C.": "team-lopburi-city",
  "Mahasarakham SBT F.C.": "team-mahasarakham",
  "Marines F.C. (Thailand)": "team-marines-eureka",
  "Muangthong United F.C.": "team-muang-thong-utd",
  "Nakhon Pathom United F.C.": "team-nakhon-pathom",
  "Nakhon Ratchasima F.C.": "team-nakhon-ratchasima",
  "Nakhon Si United F.C.": "team-nakhon-si",
  "Nara United F.C.": "team-nara-united",
  "Navy F.C.": "team-navy-fc",
  "Nongbua Pitchaya F.C.": "team-nong-bua-pitchaya",
  "Nonthaburi United F.C.": "team-nonthaburi-utd",
  "Pattani F.C.": "team-pattani",
  "Pattaya United F.C. (2023)": "team-pattaya-united",
  "Police Tero F.C.": "team-police-tero",
  "Port F.C.": "team-port-fc",
  "PT Prachuap F.C.": "team-prachuap",
  "PT Satun F.C.": "team-fc-satun",
  "Ratchaburi F.C.": "team-ratchaburi",
  "Rayong F.C.": "team-rayong-fc",
  "Samui United F.C.": "team-samui-united",
  "Samut Sakhon City F.C.": "team-samut-sakhon-city",
  "Saraburi United F.C.": "team-saraburi-utd",
  "Sisaket United F.C.": "team-sisaket-united",
  "Songkhla F.C.": "team-songkhla",
  "Sukhothai F.C.": "team-sukhothai",
  "Thonburi United F.C.": "team-thonburi-united",
  "Uthai Thani F.C.": "team-uthai-thani",
  "F.C. Yala": "team-fc-yala",
  // No separate article; the name matches the registry's T3 entry.
  "Udon Thani City": "team-udon-thani-city",
};

// Names that need a distinct identity even though they look similar to an
// existing registry entry.
const explicitNewIds = {
  "Phrae": "team-phrae-ts",
  "Nakhon Phanom United": "team-nakhon-phanom-united",
  "Chiangmai": "team-chiangmai-fc",
  "Chiangmai F.C.": "team-chiangmai-fc",
  "Khon Kaen": "team-khon-kaen-fc",
  "Khon Kaen F.C.": "team-khon-kaen-fc",
};

// Registry entries that already exist as T3 teams. Their imported display
// names are preserved; only the new participation is added.
const preserveExistingTeamIds = new Set([
  "team-angthong",
  "team-ban-khai-united",
  "team-burapha-united",
  "team-chachoengsao",
  "team-customs-united",
  "team-fc-satun",
  "team-futera-united",
  "team-hua-hin-city",
  "team-kanchanaburi",
  "team-kasem-bundit-u",
  "team-khon-kaen-fc-b",
  "team-krabi-fc",
  "team-lampang-fc",
  "team-lopburi-city",
  "team-marines-eureka",
  "team-nakhon-phanom",
  "team-nan-city",
  "team-nara-united",
  "team-navy-fc",
  "team-nonthaburi-utd",
  "team-phayao-fc",
  "team-phrae-utd",
  "team-phuket-andaman",
  "team-roi-et-fc",
  "team-royal-thai-af",
  "team-royal-thai-army",
  "team-samut-sakhon-city",
  "team-saraburi-utd",
  "team-suphan-buri-fc",
  "team-thonburi-united",
  "team-udon-thani-city",
  "team-fc-yala",
  "team-samui-united",
  "team-chiang-rai-utd-b",
  "team-buriram-res",
  "team-chiang-mai-res",
]);

// Display names that need a qualifier to stay distinct from a similarly named
// registry entry.
const displayOverrides = {
  "team-phrae-ts": "Phrae (TS)",
  "team-chiangmai-fc": "Chiangmai F.C.",
  "team-khon-kaen-fc": "Khon Kaen F.C.",
  "team-north-bangkok-university": "North Bangkok University F.C.",
};

for (const [competitionId, source] of Object.entries(cupSources.competitions)) {
  for (const entry of source.teams) {
    const key = entry.target || `TEXT:${entry.name}`;
    const teamId =
      existingClubIds[entry.target] ||
      existingClubIds[entry.name] ||
      explicitNewIds[entry.target] ||
      explicitNewIds[entry.name] ||
      `team-${slug(entry.target || entry.name)}`;
    if (!teamId || teamId === "team-") {
      throw new Error(`could not build team id for ${key}`);
    }
    if (!teams.has(teamId) && !preserveExistingTeamIds.has(teamId)) {
      const hasArticle = Boolean(entry.target);
      const displayName = displayOverrides[teamId] || entry.name;
      addTeam({
        team_id: teamId,
        display_name: displayName,
        thai_name: null,
        english_name: displayName,
        aliases: [entry.name, ...(entry.target ? [entry.target] : [])],
        organization_type: "club",
        province: null,
        verification_status: hasArticle
          ? "verified_secondary"
          : "discovered_unverified",
        first_seen_at: NOW,
        last_seen_at: NOW,
        notes: hasArticle
          ? `Club identity linked to the English Wikipedia article ${entry.target}.`
          : `Name as listed in the ${source.source_title} results; no separate club article located.`,
      });
    }
    addParticipation(
      newParticipation({
        teamId,
        competitionId,
        seasonId: source.season_id,
        ageGroup: null,
        divisionType: "cup",
        source: {
          url: source.source_url,
          title: source.source_title,
          publisher: source.source_publisher,
          date: source.source_date,
        },
        verification: "verified_secondary",
        notes: `One of ${source.entered_clubs} clubs entered in the ${source.source_title}; entry and match results imported from the same Wikipedia page; secondary source, official confirmation still preferred.`,
      }),
    );
  }
}

/* ------------------------------------------------------------------ */
/* Thai Champion Cup 2026                                              */
/* ------------------------------------------------------------------ */

const championSource = {
  url: "https://en.wikipedia.org/wiki/2026%E2%80%9327_Buriram_United_F.C._season",
  title: "2026-27 Buriram United F.C. season",
  publisher: "Wikipedia",
  date: "2026-06-25",
};
for (const [teamId, notes] of [
  ["team-buriram-united", "2025-26 Thai League 1 champion and FA Cup winner; qualifies for the 2026 Champions Cup."],
  ["team-port-fc", "2025-26 Thai League 1 runner-up; opponents in the 2026 Champions Cup because Buriram won both the league and FA Cup."],
]) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-champion-cup",
      seasonId: "2026",
      ageGroup: null,
      divisionType: "cup",
      source: championSource,
      notes,
    }),
  );
}

/* ------------------------------------------------------------------ */
/* School teams                                                        */
/* ------------------------------------------------------------------ */

const schoolTeams = [
  ["team-school-assumption", "Assumption College", "อัสสัมชัญ"],
  ["team-school-assumption-thonburi", "Assumption College Thonburi", "อัสสัมชัญธนบุรี"],
  ["team-school-assumption-sriracha", "Assumption College Sriracha", "อัสสัมชัญศรีราชา"],
  ["team-school-suankularb", "Suankularb Wittayalai School", "สวนกุหลาบวิทยาลัย"],
  ["team-school-debsirin", "Debsirin School", "เทพศิรินทร์"],
  ["team-school-debsirin-chiangmai", "Debsirin Chiang Mai School", "เทพศิรินทร์เชียงใหม่"],
  ["team-school-bangkok-christian", "Bangkok Christian College", "กรุงเทพคริสเตียนวิทยาลัย"],
  ["team-school-chai-nat-pao", "Chai Nat Provincial Administrative Organization School", "องค์การบริหารส่วนจังหวัดชัยนาท"],
  ["team-school-wat-sutthi-wararam", "Wat Sutthi Wararam School", "วัดสุทธิวราราม"],
  ["team-school-nakhon-sawan-municipality", "Nakhon Sawan Municipality Sports School", "กีฬาเทศบาลนครนครสวรรค์"],
  ["team-school-tha-kham", "Tha Kham Pittayakhom School", "ท่าข้ามพิทยาคม"],
  ["team-school-pothinimit", "Pothinimit Witthayakhom School", "โพธินิมิตวิทยาคม"],
  ["team-school-sarawittaya", "Sarawittaya School", "สารวิทยา"],
  ["team-school-chalerm-phrakiat-samut-sakhon", "Chalerm Phrakiat Somdet Phra Srinagarindra School, Samut Sakhon", "เฉลิมพระเกียรติสมเด็จพระศรีนครินทร์ สมุทรสาคร"],
  ["team-school-mathayom-6-udon", "Mathayom Thetsaban 6 Nakhon Udon Thani School", "มัธยมเทศบาล 6 นครอุดรธานี"],
  ["team-school-wachiralai", "Wachiralai School", "วชิราลัย"],
  ["team-school-ratchasima", "Ratchasima Wittayalai School", "ราชสีมาวิทยาลัย"],
  ["team-school-pathum-khongkha", "Pathum Khongkha School", "ปทุมคงคา"],
  ["team-school-phichayabundit", "Phichayabundit School", "พิชญบัณฑิต"],
  ["team-school-ratchawinit-bangkaew", "Ratchawinit Bang Kaeo School", "ราชวินิตบางแก้ว"],
  ["team-school-wichutit", "Wichutit School", "วิชูทิศ"],
  ["team-school-suan-pa-khao-cha-ang", "Suan Pa Khao Cha-ang School", "สวนป่าเขาชะอางค์"],
  ["team-school-kanthararom", "Kanthararom School", "กันทรารมณ์"],
  ["team-school-khon-kaen-sports", "Khon Kaen Sports School", "กีฬาจังหวัดขอนแก่น"],
  ["team-school-mathayom-taksin-rayong", "Mathayom Taksin Rayong School", "มัธยมตากสินระยอง"],
  ["team-school-nakhon-pathom-municipality", "Nakhon Pathom Municipality Sports School", "กีฬาเทศบาลนครนครปฐม"],
  ["team-school-suphanburi-sports", "Suphanburi Sports School", "กีฬาจังหวัดสุพรรณบุรี"],
  ["team-school-phanthai-norasing", "Phanthai Norasing Witthaya School", "พันท้ายนรสิงห์วิทยา"],
  ["team-school-amphawan", "Amphawan Witthayalai School", "อัมพวันวิทยาลัย"],
  ["team-school-nakhon-si-lamduan", "Nakhon Si Lamduan Witthaya School", "นครศรีลำดวนวิทยา"],
  ["team-school-ban-phraek", "Ban Phraek Prachasan School", "บ้านแพรกประชาสรรค์"],
  ["team-school-surasakmontri", "Surasakmontri School", "สุรศักดิ์มนตรี"],
  ["team-school-sisaket-sports", "Sisaket Sports School", "กีฬาจังหวัดศรีสะเกษ"],
  ["team-school-songkhla-pao", "Songkhla Provincial Administrative Organization Pittayanusorn School", "องค์การบริหารส่วนจังหวัดสงขลาพิทยานุสรณ์"],
  ["team-school-yala-sports", "Yala Sports School", "กีฬาจังหวัดยะลา"],
  ["team-school-hua-hin-witthayakhom", "Hua Hin Witthayakhom School", "หัวหินวิทยาคม"],
  ["team-school-nakhon-ratchasima-municipality", "Nakhon Ratchasima Municipality Sports School", "กีฬาเทศบาลนครนครราชสีมา"],
  ["team-school-wat-nawong", "Wat Nawong School", "วัดนาวง"],
  ["team-school-nakhon-nonthaburi-6", "Nakhon Nonthaburi Witthaya 6 Sports School", "กีฬานครนนท์วิทยา 6"],
  ["team-school-nakhon-sawan-provincial", "Nakhon Sawan Provincial Sports School", "กีฬาจังหวัดนครสวรรค์"],
  ["team-school-nonthaburi-wittayalai", "Nonthaburi Wittayalai School", "นนทบุรีวิทยาลัย"],
  ["team-school-khattiyawongsa", "Khattiyawongsa School", "ขัติยะวงษา"],
  ["team-school-phra-mae-mari-sathorn", "Phra Mae Mari Sathorn School", "พระแม่มารีสาทร"],
  ["team-school-satri-witthaya-2", "Satri Witthaya 2 School", "สตรีวิทยา 2"],
  ["team-school-plutaluang-witthaya", "Plutaluang Witthaya School", "พลูตาหลวงวิทยา"],
  ["team-school-thetsaban-tha-klong-1", "Thetsaban Tha Klong 1 School", "เทศบาลท่าโขลง 1"],
  ["team-school-sarakham-pittayakhom", "Sarakham Pittayakhom School", "สารคามพิทยาคม"],
  ["team-school-kaen-nakhon", "Kaen Nakhon Wittayalai School", "แก่นนครวิทยาลัย"],
  ["team-school-wat-thong-samrit", "Wat Thong Samrit School", "วัดทองสัมฤทธิ์"],
  ["team-school-ratchawinit-bangkhen", "Ratchawinit Bang Khen School", "ราชวินิตบางเขน"],
];

addTeam(
  newTeam(
    "team-college-ratchadamnoen",
    "Panitchayakan Rajdamnern Technological College",
    "วิทยาลัยเทคโนโลยีพณิชยการราชดำเนิน",
    "college",
    "College team identity curated from the 2026 DPE WOW Cup Type B U18 draw; English name follows the owner-supplied 2026 Thai College Cup bracket.",
  ),
);

// Correct the Thai spelling of the Kanthararom school (กันทรารมณ์).
for (const entry of schoolTeams) {
  if (entry[0] === "team-school-kanthararom") entry[2] = "กันทรารมณ์";
}

for (const [teamId, displayName, thaiName] of schoolTeams) {
  addTeam(
    newTeam(
      teamId,
      displayName,
      thaiName,
      "school",
      "School team identity curated from public competition reporting.",
    ),
  );
}

const typeASource = {
  url: "https://www.thaipost.net/sport-news/991753",
  title: "กรมพลศึกษา เดลินิวส์ คัพ 2026 — 16 teams in Type A U18 and U16",
  publisher: "Thai Post",
  date: "2026-05-06",
};

const typeA18 = [
  "team-school-suankularb",
  "team-school-assumption-thonburi",
  "team-school-debsirin",
  "team-school-bangkok-christian",
  "team-school-chai-nat-pao",
  "team-school-assumption-sriracha",
  "team-school-ratchasima",
  "team-school-pathum-khongkha",
  "team-school-wat-sutthi-wararam",
  "team-school-nakhon-sawan-municipality",
  "team-school-tha-kham",
  "team-school-pothinimit",
  "team-school-sarawittaya",
  "team-school-chalerm-phrakiat-samut-sakhon",
  "team-school-mathayom-6-udon",
  "team-school-wachiralai",
];

const typeA16 = [
  "team-school-assumption",
  "team-school-suankularb",
  "team-school-assumption-thonburi",
  "team-school-phichayabundit",
  "team-school-ratchawinit-bangkaew",
  "team-school-chai-nat-pao",
  "team-school-wichutit",
  "team-school-wat-sutthi-wararam",
  "team-school-debsirin-chiangmai",
  "team-school-bangkok-christian",
  "team-school-suan-pa-khao-cha-ang",
  "team-school-pothinimit",
  "team-school-kanthararom",
  "team-school-debsirin",
  "team-school-khon-kaen-sports",
  "team-school-mathayom-taksin-rayong",
];

for (const [teamId, ageGroup] of [
  ...typeA18.map((id) => [id, "U18"]),
  ...typeA16.map((id) => [id, "U16"]),
]) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-school-league-type-a",
      seasonId: `2026-${ageGroup.toLowerCase()}`,
      ageGroup,
      divisionType: "league",
      source: typeASource,
      notes: `Listed among the 16 teams in the 2026 (2569) Type A ${ageGroup} draw.`,
    }),
  );
}

const typeB18Source = {
  url: "https://www.matichon.co.th/sport/footballlocal/news_5836043",
  title: "DPE WOW Cup 2026 — 18 ปี ข (16 teams)",
  publisher: "Matichon",
  date: "2026-08-06",
};

const typeB18 = [
  "team-school-ratchawinit-bangkaew",
  "team-college-ratchadamnoen",
  "team-school-phichayabundit",
  "team-school-wichutit",
  "team-school-hua-hin-witthayakhom",
  "team-school-nakhon-ratchasima-municipality",
  "team-school-wat-nawong",
  "team-school-nakhon-nonthaburi-6",
  "team-school-suphanburi-sports",
  "team-school-phanthai-norasing",
  "team-school-assumption",
  "team-school-nakhon-pathom-municipality",
  "team-school-suan-pa-khao-cha-ang",
  "team-school-khon-kaen-sports",
  "team-school-kanthararom",
  "team-school-mathayom-taksin-rayong",
];

for (const teamId of typeB18) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-school-league-type-b",
      seasonId: "2026-u18",
      ageGroup: "U18",
      divisionType: "league",
      source: typeB18Source,
      notes:
        "One of the 16 teams in the 2026 (2569) Type B U18 draw (DPE WOW Cup 2026).",
    }),
  );
}

const typeB16Source = {
  url: "https://www.facebook.com/footballromcom/posts/1767809752018325/",
  title: "DPE WOW Cup 2026 — 16 ปี ข (16 teams)",
  publisher: "Football Romcom (Facebook)",
  date: "2026-11-01",
};

const typeB16 = [
  "team-school-pathum-khongkha",
  "team-school-nakhon-sawan-provincial",
  "team-school-mathayom-6-udon",
  "team-school-nonthaburi-wittayalai",
  "team-school-khattiyawongsa",
  "team-school-suphanburi-sports",
  "team-school-phra-mae-mari-sathorn",
  "team-school-tha-kham",
  "team-school-nakhon-pathom-municipality",
  "team-school-satri-witthaya-2",
  "team-school-assumption-sriracha",
  "team-school-nakhon-sawan-municipality",
  "team-school-plutaluang-witthaya",
  "team-school-nakhon-ratchasima-municipality",
  "team-school-thetsaban-tha-klong-1",
  "team-school-sarakham-pittayakhom",
];

for (const teamId of typeB16) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-school-league-type-b",
      seasonId: "2026-u16",
      ageGroup: "U16",
      divisionType: "league",
      source: typeB16Source,
      verification: "discovered_unverified",
      notes:
        "One of the 16 teams in the 2026 (2569) Type B U16 list published by a public youth-football page; the official DPE draw document was not captured.",
    }),
  );
}

// Type A U14: the 2025 (2568) edition is the latest complete public list.
const typeA14Source = {
  url: "https://www.siamsport.co.th/football-thailand/th-other/60160/",
  title: "ฟุตบอลเยาวชน 14 ปี ก. กรมพลศึกษา 2567 (16 teams)",
  publisher: "Siamsport",
  date: "2024-08-23",
};

const typeA14 = [
  "team-school-bangkok-christian",
  "team-school-debsirin",
  "team-school-tha-kham",
  "team-school-mathayom-taksin-rayong",
  "team-school-nakhon-pathom-municipality",
  "team-school-pothinimit",
  "team-school-phichayabundit",
  "team-school-suankularb",
  "team-school-pathum-khongkha",
  "team-school-khon-kaen-sports",
  "team-school-assumption-sriracha",
  "team-school-kaen-nakhon",
  "team-school-assumption-thonburi",
  "team-school-phanthai-norasing",
  "team-school-wat-thong-samrit",
  "team-school-ratchawinit-bangkhen",
];

for (const teamId of typeA14) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-school-league-type-a",
      seasonId: "2026-u14",
      ageGroup: "U14",
      divisionType: "league",
      source: typeA14Source,
      notes:
        "Latest complete public Type A U14 list located (2024/2567 edition); the 2025/2568 and 2026/2569 full lists were not published in accessible text.",
    }),
  );
}

/* ------------------------------------------------------------------ */
/* Youth cups                                                          */
/* ------------------------------------------------------------------ */

// Coke Cup U17: 12 national-final teams (regional champions and runners-up).
addTeam(
  newTeam(
    "team-assumption-united",
    "Assumption United",
    "อัสสัมชัญ ยูไนเต็ด",
    "club",
    "Club identity curated from the 2026 Coke Cup U17 national final list.",
  ),
);
addTeam(
  newTeam(
    "team-maejo-united",
    "Maejo United",
    "แม่โจ้ ยูไนเต็ด",
    "club",
    "Club identity curated from the 2026 Coke Cup U17 national final list.",
  ),
);

const cokeCupSource = {
  url: "https://siamsport.co.th/football-thailand/th-other/107933",
  title: "Coke Cup U17 Thailand Football Championship 2026 — 12 national-final teams",
  publisher: "Siamsport",
  date: "2026-08-14",
};

const cokeCupTeams = [
  "team-chainat",
  "team-nakhon-pathom",
  "team-chonburi",
  "team-rayong-fc",
  "team-school-kanthararom",
  "team-school-sisaket-sports",
  "team-school-songkhla-pao",
  "team-school-yala-sports",
  "team-chiangmai-fc",
  "team-maejo-united",
  "team-assumption-united",
  "team-bangkok-utd",
];

for (const teamId of cokeCupTeams) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-youth-fa-cup-u17",
      seasonId: "2026",
      ageGroup: "U17",
      divisionType: "cup",
      source: cokeCupSource,
      notes:
        "One of the 12 regional champions/runners-up in the 2026 Coke Cup U17 national championship round.",
    }),
  );
}

// 7HD Champion Cup U18: eight schools in the 2026 national final round.
const sevenHdSource = {
  url: "https://thethaiger.com/th/news/1634775",
  title: "แชมป์กีฬา 7HD แชมเปียน คัพ 2026 — 8 final teams",
  publisher: "The Thaiger",
  date: "2026-11-24",
};

const sevenHdTeams = [
  "team-school-wichutit",
  "team-school-wat-sutthi-wararam",
  "team-school-tha-kham",
  "team-school-assumption-sriracha",
  "team-school-assumption-thonburi",
  "team-school-assumption",
  "team-school-debsirin",
  "team-school-kanthararom",
];

for (const teamId of sevenHdTeams) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-youth-league-cup-u18",
      seasonId: "2026",
      ageGroup: "U18",
      divisionType: "cup",
      source: sevenHdSource,
      notes:
        "One of the 8 schools in the 2026 7HD Champion Cup national final round.",
    }),
  );
}

// Jaturamitr Samakkhi U18: four traditional schools.
const jaturamitrSource = {
  url: "https://matichon.co.th/sport/news_5456292",
  title: "จตุรมิตรสามัคคี ครั้งที่ 31 (2025)",
  publisher: "Matichon",
  date: "2025-11-15",
};

const jaturamitrTeams = [
  "team-school-assumption",
  "team-school-bangkok-christian",
  "team-school-suankularb",
  "team-school-debsirin",
];

for (const teamId of jaturamitrTeams) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-youth-champion-cup-u18",
      seasonId: "2026",
      ageGroup: "U18",
      divisionType: "cup",
      source: jaturamitrSource,
      notes:
        "One of the 4 traditional schools in the 31st Jaturamitr Samakkhi (2025). No 2026 football edition was held.",
    }),
  );
}

/* ------------------------------------------------------------------ */
/* Thai College Cup (U19) — corrected 2026 knockout bracket            */
/* ------------------------------------------------------------------ */

const collegeCupSource = {
  url: COLLEGE_CUP_SOURCE_URL,
  title:
    "Thai College Cup (U19) 2026 — quarter-final participants, 14–15 August 2026",
  publisher:
    "Owner-supplied bracket image; Thawee SC official report corroborates QF/SF scores",
  date: "2026-08-15",
};

const collegeCupTeams = [
  ["team-college-ratchadamnoen", "Panitchayakan Rajdamnern Technological College", "วิทยาลัยเทคโนโลยีพณิชยการราชดำเนิน"],
  ["team-college-rajasitharam", "Rajasitharam Technical College", "วิทยาลัยเทคนิคราชสิทธาราม"],
  ["team-college-kanchanaburi", "Kanchanaburi Technical College", "วิทยาลัยเทคนิคกาญจนบุรี"],
  ["team-college-thawee-sc", "Thawee SC Business Administration and Technological College", "วิทยาลัยเทคโนโลยีบริหารธุรกิจทวีเอสซี"],
  ["team-college-chonburi-industrial", "Chonburi Industrial Technology College", "วิทยาลัยเทคโนโลยีอุตสาหกรรมชลบุรี"],
  ["team-college-nakhon-si-thammarat", "Nakhon Si Thammarat Technical College", "วิทยาลัยเทคนิคนครศรีธรรมราช"],
  ["team-college-lampang", "Lampang Technical College", "วิทยาลัยเทคนิคลำปาง"],
  ["team-college-laem-chabang", "Laem Chabang Engineering Technology College", "วิทยาลัยเทคโนโลยีวิศวกรรมแหลมฉบัง"],
];

for (const [teamId, displayName, thaiName] of collegeCupTeams) {
  addTeam(
    newTeam(
      teamId,
      displayName,
      thaiName,
      "college",
      "College team identity curated from the owner-supplied 14–15 August 2026 Thai College Cup bracket.",
    ),
  );
}

for (const [teamId] of collegeCupTeams) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-college-cup-u19",
      seasonId: "2026",
      ageGroup: "U19",
      divisionType: "cup",
      source: collegeCupSource,
      notes:
        "Quarter-final participant in the corrected 2026 Thai College Cup bracket. Yasothon is not part of this bracket.",
    }),
  );
}

/* ------------------------------------------------------------------ */
/* University Cup                                                      */
/* ------------------------------------------------------------------ */

const universityTeams = [
  ["team-university-thailand-national-sports", "Thailand National Sports University", "มหาวิทยาลัยการกีฬาแห่งชาติ"],
  ["team-university-chulalongkorn", "Chulalongkorn University", "จุฬาลงกรณ์มหาวิทยาลัย"],
  ["team-university-bangkok-thonburi", "Bangkok Thonburi University", "มหาวิทยาลัยกรุงเทพธนบุรี"],
  ["team-university-north-bangkok", "North Bangkok University", "มหาวิทยาลัยนอร์ทกรุงเทพ"],
  ["team-university-mahidol", "Mahidol University", "มหาวิทยาลัยมหิดล"],
  ["team-university-thammasat", "Thammasat University", "มหาวิทยาลัยธรรมศาสตร์"],
  ["team-university-thaksin", "Thaksin University", "มหาวิทยาลัยทักษิณ"],
  ["team-kasem-bundit-u", "Kasem Bundit University", "มหาวิทยาลัยเกษมบัณฑิต"],
];

for (const [teamId, displayName, thaiName] of universityTeams) {
  addTeam(
    newTeam(
      teamId,
      displayName,
      thaiName,
      "university",
      "University team identity curated from public Chang U-Champion Cup 2026 reporting.",
    ),
  );
}

const universitySource = {
  url: "https://edu-today.com/btu-215",
  title: "Chang U-Champion Cup 2026 — knockout-stage participants",
  publisher: "Education Today",
  date: "2026-06-15",
};

for (const [teamId] of universityTeams) {
  addParticipation(
    newParticipation({
      teamId,
      competitionId: "thai-university-cup-u23",
      seasonId: "2026",
      ageGroup: "U23",
      divisionType: "cup",
      source: universitySource,
      notes:
        "Confirmed knockout-stage participant in the 2026 Chang U-Champion Cup. The full 16-team field was not published in accessible text form.",
    }),
  );
}

/* ------------------------------------------------------------------ */

const usedTeamIds = new Set(participations.map((participation) => participation.team_id));
curated.teams = [...teams.values()]
  .filter((team) => usedTeamIds.has(team.team_id))
  .sort((a, b) => a.team_id.localeCompare(b.team_id));
curated.participations = participations.sort((a, b) =>
  a.participation_id.localeCompare(b.participation_id),
);
curated.rosters = Object.entries(rosterDefinitions)
  .map(([key, definition]) => {
    const [competition_id, season_id] = key.split("|");
    const team_count = curated.participations.filter(
      (participation) =>
        participation.competition_id === competition_id &&
        participation.season_id === season_id,
    ).length;
    return {
      competition_id,
      season_id,
      age_group: definition.age_group ?? null,
      displayed_season: definition.displayed_season,
      source_season: definition.source_season,
      roster_status: definition.roster_status,
      source_url: definition.source_url ?? null,
      source_checked_at: NOW,
      note: definition.note ?? "",
      team_count,
    };
  })
  .sort((a, b) =>
    `${a.competition_id}|${a.season_id}`.localeCompare(
      `${b.competition_id}|${b.season_id}`,
    ),
  );
curated.generated_at = NOW;
curated.notes =
  "Curated public-source team participations. National cups use English Wikipedia season articles; school, youth and university lists use the news reports recorded in each participation note. Secondary sources are not official confirmation.";

fs.writeFileSync(CURATED, JSON.stringify(curated, null, 2), "utf8");

console.log(
  JSON.stringify(
    {
      teams: curated.teams.length,
      participations: curated.participations.length,
      rosters: curated.rosters.length,
      file: CURATED,
    },
    null,
    2,
  ),
);
