/**
 * Build the curated secondary-source team list for the 2025/26 national
 * leagues. Wikipedia is used as a secondary source only; official sources
 * remain preferred for future updates.
 *
 * Run: node tools/build_verified_teams.mjs
 */
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(import.meta.dirname, "..");
const seed = JSON.parse(fs.readFileSync(path.join(ROOT, "data", "seed.json"), "utf8"));

const mappings = {
  "thai-league-1": {
    source_url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_1",
    source_title: "2025–26 Thai League 1",
    source_date: "2026-05-10",
    teams: {
      "team-buriram-united": "Buriram United",
      "team-port-fc": "Port",
      "team-ratchaburi": "Ratchaburi",
      "team-bangkok-utd": "Bangkok United",
      "team-pathum-united": "BG Pathum United",
      "team-prachuap": "PT Prachuap",
      "team-rayong-fc": "Rayong",
      "team-ayutthaya-utd": "Ayutthaya United",
      "team-chonburi": "Chonburi",
      "team-chiangrai-utd": "Chiangrai United",
      "team-uthai-thani": "Uthai Thani",
      "team-sukhothai": "Sukhothai",
      "team-lamphun-warrior": "Lamphun Warriors",
      "team-kanchanaburi": "Kanchanaburi Power",
      "team-muang-thong-utd": "Muangthong United",
      "team-nakhon-ratchasima": "Nakhon Ratchasima"
    }
  },
  "thai-league-2": {
    source_url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_2",
    source_title: "2025–26 Thai League 2",
    source_date: "2026-05-23",
    teams: {
      "team-raisalai-united": "Rasisalai United",
      "team-sisaket-united": "Sisaket United",
      "team-pattani": "Pattani",
      "team-nong-bua-pitchaya": "Nongbua Pitchaya",
      "team-police-tero": "Police Tero",
      "team-mahasarakham": "Mahasarakham",
      "team-chainat": "Chainat Hornbill",
      "team-phrae": "Phrae United",
      "team-khon-kaen-united": "Khonkaen United",
      "team-songkhla": "Songkhla",
      "team-trat-fc": "Trat",
      "team-chanthaburi": "Chanthaburi",
      "team-pattaya-united": "Pattaya United",
      "team-chiang-mai-fc": "Chiangmai United",
      "team-kasetsart": "Kasetsart",
      "team-nakhon-pathom": "Nakhon Pathom United",
      "team-nakhon-si": "Nakhon Si United",
      "team-bangkok-fc": "Bangkok"
    }
  }
};

const now = "2026-11-25";
const teams = [];
const participations = [];

for (const [competitionId, mapping] of Object.entries(mappings)) {
  const competition = seed.competitions.find((entry) => entry.id === competitionId);
  if (!competition) throw new Error(`missing competition ${competitionId}`);
  const season = competition.seasons[0];
  const standings = season.groups[0].standings;
  for (const row of standings) {
    const officialName = mapping.teams[row.team_id];
    if (!officialName) throw new Error(`missing official name for ${row.team_id}`);
    const aliases = [...new Set([row.team_name, officialName])];
    teams.push({
      team_id: row.team_id,
      display_name: row.team_name,
      thai_name: null,
      english_name: officialName,
      aliases,
      organization_type: "club",
      province: null,
      verification_status: "verified_secondary",
      first_seen_at: now,
      last_seen_at: now,
      notes: "Team identity confirmed against a secondary 2025/26 competition table."
    });
    participations.push({
      participation_id: `part-${competitionId}-${season.id}-${row.team_id}`,
      team_id: row.team_id,
      competition_id: competitionId,
      season_id: season.id,
      age_group: null,
      group_id: "overall",
      division_type: "league",
      source_url: mapping.source_url,
      source_title: mapping.source_title,
      source_publisher: "Wikipedia",
      source_date: mapping.source_date,
      verification_status: "verified_secondary",
      first_seen_at: now,
      last_seen_at: now,
      notes: "Listed in the 2025/26 team table; secondary source, official confirmation still preferred."
    });
  }
}

fs.mkdirSync(path.join(ROOT, "data", "discovery"), { recursive: true });
fs.writeFileSync(
  path.join(ROOT, "data", "discovery", "curated_verified_teams.json"),
  JSON.stringify({
    generated_at: now,
    source_priority: "secondary",
    notes: "Curated secondary-source team participations for 2025/26 Thai League 1 and 2. Do not treat as official confirmation.",
    teams,
    participations
  }, null, 2),
  "utf8"
);

console.log(JSON.stringify({
  teams: teams.length,
  participations: participations.length,
  file: path.join(ROOT, "data", "discovery", "curated_verified_teams.json")
}, null, 2));
