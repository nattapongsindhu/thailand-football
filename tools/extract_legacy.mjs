/**
 * One-off development extractor.
 *
 * Reads the owner-supplied reference HTML, extracts the embedded football data,
 * and writes project-owned JSON snapshots. The reference application is never
 * modified and is not copied into this project.
 *
 * Run: node tools/extract_legacy.mjs
 */
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";

const ROOT = path.resolve(import.meta.dirname, "..");
const REF = process.env.TF_REFERENCE_FILE
  || "C:\\Users\\BMW\\Videos\\Thai League (T1-T5)\\thai_league_05.html";

const source = fs.readFileSync(REF, "utf8");
const sha256 = crypto.createHash("sha256").update(source).digest("hex");

const dataCode = source.slice(source.indexOf("const T1 = ["), source.indexOf("function buildTeamStats"));
const legacy = new Function(`${dataCode};return {T1,T2,T3G,T4G,TAG,LEAGUES,RESULTS,FIXTURES,NEWS,SCORERS};`)();

const cupCode = source.slice(source.indexOf("const CUP_DATA = {"), source.indexOf("function renderCup"));
const cups = new Function(`${cupCode};return CUP_DATA;`)();

const slug = (value) => value
  .normalize("NFKD")
  .replace(/[\u0300-\u036f]/g, "")
  .replace(/&/g, " and ")
  .replace(/['’]/g, "")
  .replace(/[^A-Za-z0-9\u0E00-\u0E7F]+/g, "-")
  .replace(/^-+|-+$/g, "")
  .toLowerCase();

const ids = new Map();
const uniqueId = (prefix, name) => {
  let base = `${prefix}-${slug(name)}`;
  if (!ids.has(base)) { ids.set(base, 1); return base; }
  let n = ids.get(base) + 1;
  ids.set(base, n);
  return `${base}-${n}`;
};

const seasonId = (label) => label.replace("/", "-");

const competitionDefs = [
  {
    id: "thai-league-1", name: "Thai League 1", short_name: "T1", section: "National League",
    category: "Senior", kind: "league", age_group: null, sort_order: 10, active: true,
    official_name: "Thai League 1", aliases: ["Thai League 1"], group_label: null,
    description: "Thailand's top professional football division.",
    source_ids: ["legacy-import", "thaileague", "wikipedia-t1"],
  },
  {
    id: "thai-league-2", name: "Thai League 2", short_name: "T2", section: "National League",
    category: "Senior", kind: "league", age_group: null, sort_order: 20, active: true,
    official_name: "Thai League 2", aliases: ["Thai League 2"], group_label: null,
    description: "Thailand's second professional football division.",
    source_ids: ["legacy-import", "thaileague", "wikipedia-t2"],
  },
  {
    id: "thai-league-3", name: "Thai League 3", short_name: "T3", section: "National League",
    category: "Senior", kind: "league", age_group: null, sort_order: 30, active: true,
    official_name: "Thai League 3", aliases: ["Thai League 3", "Thai League 3 (Region)", "Thai League 3 (Regions)"],
    group_label: null,
    description: "Thailand's third football level, organised in regional groups.",
    source_ids: ["legacy-import", "thaileague", "wikipedia-t3"],
  },
  {
    id: "thai-fa-cup", name: "Thai FA Cup", short_name: "FA Cup", section: "National League",
    category: "Senior", kind: "cup", age_group: null, sort_order: 40, active: true,
    official_name: "Thai FA Cup", aliases: ["FA Cup", "Chang FA Cup"], group_label: null,
    description: "National knockout cup. Sponsor/legacy name: Chang FA Cup.",
    source_ids: ["legacy-import", "fat", "wikipedia-fac"],
  },
  {
    id: "thai-league-cup", name: "Thai League Cup", short_name: "League Cup", section: "National League",
    category: "Senior", kind: "cup", age_group: null, sort_order: 50, active: true,
    official_name: "Thai League Cup", aliases: ["League Cup", "Muang Thai Cup"], group_label: null,
    description: "National league cup. Sponsor/legacy name: Muang Thai Cup.",
    source_ids: ["legacy-import", "fat", "wikipedia-lc"],
  },
  {
    id: "thai-champion-cup", name: "Thai Champion Cup", short_name: "Champion Cup", section: "National League",
    category: "Senior", kind: "cup", age_group: null, sort_order: 60, active: true,
    official_name: "Thailand Champions Cup", aliases: ["Champions Cup", "Thai Champions Cup", "Thailand Champions Cup"],
    group_label: null, description: "National super cup between the Thai League 1 champion and FA Cup winner.",
    source_ids: ["legacy-import", "fat"],
  },
  {
    id: "thai-youth-league-u18", name: "Thai Youth League 1 (U18)", short_name: "U18",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "league",
    age_group: "U18", sort_order: 110, active: true,
    official_name: "Thai Youth League 1 (U18)", aliases: ["Thailand Youth League U18"],
    group_label: "Youth League (FAT & SAT)",
    description: "Youth competition organised under the FA Thailand and Sports Authority of Thailand structure.",
    source_ids: ["owner-reference", "fat"],
  },
  {
    id: "thai-youth-league-u16", name: "Thai Youth League 2 (U16)", short_name: "U16",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "league",
    age_group: "U16", sort_order: 120, active: true,
    official_name: "Thai Youth League 2 (U16)", aliases: ["Thailand Youth League U16"],
    group_label: "Youth League (FAT & SAT)",
    description: "Youth competition organised under the FA Thailand and Sports Authority of Thailand structure.",
    source_ids: ["owner-reference", "fat"],
  },
  {
    id: "thai-youth-league-u14", name: "Thai Youth League 3 (U14)", short_name: "U14",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "league",
    age_group: "U14", sort_order: 130, active: true,
    official_name: "Thai Youth League 3 (U14)", aliases: ["Thailand Youth League U14"],
    group_label: "Youth League (FAT & SAT)",
    description: "Youth competition organised under the FA Thailand and Sports Authority of Thailand structure.",
    source_ids: ["owner-reference", "fat"],
  },
  {
    id: "thai-school-league-type-a", name: "Thai School League — Type A", short_name: "Type A",
    section: "School League (DPE)", category: "Youth", kind: "league",
    age_group: null, sort_order: 210, active: true,
    official_name: "Thai School League — Type A",
    aliases: ["ฟุตบอลนักเรียน กรมพลฯ ประเภท ก", "Thai School League Type A"],
    group_label: "School League (DPE)",
    description: "School football competition organised by the Department of Physical Education; Type A category.",
    source_ids: ["owner-reference", "dpe"],
  },
  {
    id: "thai-school-league-type-b", name: "Thai School League — Type B", short_name: "Type B",
    section: "School League (DPE)", category: "Youth", kind: "league",
    age_group: null, sort_order: 220, active: true,
    official_name: "Thai School League — Type B",
    aliases: ["ฟุตบอลนักเรียน กรมพลฯ ประเภท ข", "Thai School League Type B"],
    group_label: "School League (DPE)",
    description: "School football competition organised by the Department of Physical Education; Type B category.",
    source_ids: ["owner-reference", "dpe"],
  },
  {
    id: "thai-school-league-general", name: "Thai School League — General", short_name: "General",
    section: "School League (DPE)", category: "Youth", kind: "league",
    age_group: null, sort_order: 230, active: true,
    official_name: "Thai School League — General",
    aliases: ["ฟุตบอลนักเรียน กรมพลฯ ประเภท ทั่วไป", "Thai School League General"],
    group_label: "School League (DPE)",
    description: "School football competition organised by the Department of Physical Education; general division.",
    source_ids: ["owner-reference", "dpe"],
  },
  {
    id: "thai-college-cup-u19", name: "Thai College Cup (U19)", short_name: "U19",
    section: "School League (DPE)", category: "Youth", kind: "cup", age_group: "U19", sort_order: 240,
    active: true, official_name: "Thai College Cup (U19)",
    aliases: ["ฟุตบอลอาชีวะระดับประเทศ"], group_label: "School League (DPE)",
    description: "Cup competition for national vocational education institutions.",
    source_ids: ["owner-reference"],
  },
  {
    id: "thai-university-cup-u23", name: "Thai University Cup (U23)", short_name: "U23",
    section: "School League (DPE)", category: "Youth", kind: "cup", age_group: "U23", sort_order: 250,
    active: true, official_name: "Thai University Cup (U23)",
    aliases: ["ฟุตบอลมหาวิทยาลัยระดับประเทศ", "University Champions Cup"], group_label: "School League (DPE)",
    description: "Cup competition for national university institutions.",
    source_ids: ["owner-reference"],
  },
  {
    id: "thai-youth-fa-cup-u17", name: "Thai Youth FA Cup (U17)", short_name: "U17",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "cup", age_group: "U17",
    sort_order: 140, active: true, official_name: "Thai Youth FA Cup (U17)",
    aliases: ["Coke Cup", "Coke Cup U17", "Thai School FA Cup U17"],
    group_label: "Youth League (FAT & SAT)",
    description: "Youth cup under the FA Thailand structure; sponsor alias: Coke Cup.",
    source_ids: ["owner-reference"],
  },
  {
    id: "thai-youth-league-cup-u18", name: "Thai Youth League Cup (U18)", short_name: "U18",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "cup", age_group: "U18",
    sort_order: 150, active: true, official_name: "Thai Youth League Cup (U18)",
    aliases: ["7HD Champion Cup", "Channel 7HD Champion Cup", "Thai School League Cup U18"],
    group_label: "Youth League (FAT & SAT)",
    description: "Youth league cup under the FA Thailand structure; sponsor alias: 7HD Champion Cup.",
    source_ids: ["owner-reference", "ch7"],
  },
  {
    id: "thai-youth-champion-cup-u18", name: "Thai Youth Champion Cup (U18)", short_name: "U18",
    section: "Youth League (FAT & SAT)", category: "Youth", kind: "cup", age_group: "U18",
    sort_order: 160, active: true, official_name: "Thai Youth Champion Cup (U18)",
    aliases: [], group_label: "Youth League (FAT & SAT)",
    description: "Youth champion cup under the FA Thailand structure.",
    source_ids: ["owner-reference"],
  },
];

const sources = [
  {
    id: "legacy-import", title: "Owner-supplied offline reference application",
    url: null, retrieved_at: "2026-03-22", kind: "legacy",
    note: "Imported from thai_league_05.html. Values are preserved; imported rows are marked unverified unless independently checked.",
  },
  {
    id: "thaileague", title: "Thai League (official)", url: "https://www.thaileague.co.th/",
    retrieved_at: "2026-11-25", kind: "official", note: "General competition reference; not used to overwrite imported rows.",
  },
  {
    id: "fat", title: "Football Association of Thailand (official)", url: "https://www.fathailand.org/",
    retrieved_at: "2026-11-25", kind: "official", note: "General competition and cup reference.",
  },
  {
    id: "wikipedia-t1", title: "2025–26 Thai League 1 (reference)", url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_1",
    retrieved_at: "2026-11-25", kind: "secondary", note: "Secondary reference; imported snapshot was not replaced.",
  },
  {
    id: "wikipedia-t2", title: "2025–26 Thai League 2 (reference)", url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_2",
    retrieved_at: "2026-11-25", kind: "secondary", note: "Secondary reference.",
  },
  {
    id: "wikipedia-t3", title: "2025–26 Thai League 3 (reference)", url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_3",
    retrieved_at: "2026-11-25", kind: "secondary", note: "Secondary reference.",
  },
  {
    id: "wikipedia-fac", title: "2025–26 Thai FA Cup (reference)", url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_FA_Cup",
    retrieved_at: "2026-11-25", kind: "secondary", note: "Secondary reference.",
  },
  {
    id: "wikipedia-lc", title: "2025–26 Thai League Cup (reference)", url: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_Cup",
    retrieved_at: "2026-11-25", kind: "secondary", note: "Secondary reference.",
  },
  {
    id: "dpe", title: "Department of Physical Education (reference)", url: "https://www.dpe.go.th/",
    retrieved_at: "2026-11-25", kind: "official", note: "Reference for owner-defined school categories ก./ข./ค./ง.; no records imported.",
  },
  {
    id: "ch7", title: "Channel 7HD (reference)", url: "https://www.ch7.com/",
    retrieved_at: "2026-11-25", kind: "official", note: "Reference for owner display category; no records imported.",
  },
  {
    id: "owner-reference", title: "Owner-approved competition naming and categories",
    url: null, retrieved_at: "2026-11-25", kind: "owner",
    note: "Owner-defined organizational labels. Not evidence of official eligibility or a connected promotion/relegation pyramid.",
  },
];

const news = legacy.NEWS.map((n, index) => ({
  id: `news-${String(index + 1).padStart(2, "0")}`,
  title: n.title,
  summary: n.summary,
  published_label: n.date,
  source_name: n.src,
  tag: n.tag,
  emoji: n.emoji,
  competition_ids: n.lg.map((key) => ({
    t1: "thai-league-1", t2: "thai-league-2", t3: "thai-league-3",
    fac: "thai-fa-cup", lc: "thai-league-cup", cc: "thai-champion-cup",
    t4: "legacy-thai-league-4", ta: "legacy-thai-league-5",
  }[key])).filter(Boolean),
}));

const aliasesByCompetition = {
  "thai-league-2": {
    "Chainat Hornbill": "Chainat",
    "Pattaya Utd": "Pattaya United",
    "Nakhon Si Utd": "Nakhon Si",
  },
};

const activeTeamRows = [];
const addRows = (competitionId, groupId, teams) => {
  for (const team of teams) {
    activeTeamRows.push({ competitionId, groupId, name: team.name, row: team });
  }
};
addRows("thai-league-1", "overall", legacy.T1);
addRows("thai-league-2", "overall", legacy.T2);
legacy.T3G.forEach((group, index) => addRows("thai-league-3", `group-${index + 1}`, group.teams));

const teamIdByKey = new Map();
const teamIdsByCompetition = new Map();
const teamIdByName = new Map();
for (const item of activeTeamRows) {
  const teamId = uniqueId("team", item.name);
  teamIdByKey.set(`${item.competitionId}|${item.groupId}|${item.name}`, teamId);
  if (!teamIdsByCompetition.has(item.competitionId)) teamIdsByCompetition.set(item.competitionId, new Map());
  teamIdsByCompetition.get(item.competitionId).set(item.name, teamId);
  if (!teamIdByName.has(item.name)) teamIdByName.set(item.name, teamId);
}

const validationFor = (teams) => {
  if (!teams.length) return { status: "no-data", messages: ["No records imported."] };
  const sum = (key) => teams.reduce((total, t) => total + (Number(t[key]) || 0), 0);
  const messages = [];
  if (teams.some((t) => (t.w + t.d + t.l) !== t.mp)) messages.push("At least one row has MP ≠ W + D + L.");
  if (teams.some((t) => (3 * t.w + t.d) !== t.pts)) messages.push("At least one row has points ≠ 3W + D.");
  if (teams.some((t) => (t.gf - t.ga) !== t.gd)) messages.push("At least one row has GD ≠ GF − GA.");
  if (sum("w") !== sum("l")) messages.push(`Group total wins (${sum("w")}) ≠ total losses (${sum("l")}).`);
  if (sum("gf") !== sum("ga")) messages.push(`Group total GF (${sum("gf")}) ≠ total GA (${sum("ga")}).`);
  if (sum("d") % 2 !== 0) messages.push(`Group total draws (${sum("d")}) is odd.`);
  return { status: messages.length ? "failed" : "ok", messages };
};

const standingsFor = (competitionId, groupId, teams) => teams.map((t) => ({
  team_id: teamIdByKey.get(`${competitionId}|${groupId}|${t.name}`),
  team_name: t.name,
  rank: t.rank,
  played: t.mp,
  won: t.w,
  drawn: t.d,
  lost: t.l,
  goals_for: t.gf,
  goals_against: t.ga,
  goal_difference: t.gd,
  points: t.pts,
  form: t.form || [],
  zone: t.zone || "",
}));

const resolveTeam = (competitionId, name) => {
  const competitionTeams = teamIdsByCompetition.get(competitionId);
  if (competitionTeams?.has(name)) {
    return { id: competitionTeams.get(name), name, status: "resolved", issues: [] };
  }
  const alias = aliasesByCompetition[competitionId]?.[name];
  if (alias && competitionTeams?.has(alias)) {
    return { id: competitionTeams.get(alias), name: alias, status: "resolved", issues: [`Alias used: ${name} → ${alias} (unverified editorial alias).`] };
  }
  return { id: null, name, status: "unresolved", issues: [`Team is not present in the imported ${competitionId} standings.`] };
};

const matchRows = (competitionId, data, kind) => {
  const rows = [];
  for (const round of data || []) {
    for (const match of round.matches) {
      const home = resolveTeam(competitionId, match.home);
      const away = resolveTeam(competitionId, match.away);
      rows.push({
        id: uniqueId("match", `${competitionId}-${kind}-${round.round}-${match.home}-${match.away}`),
        kind,
        round_label: round.round,
        date_label: round.date,
        home_team_id: home.id,
        away_team_id: away.id,
        home_name: match.home,
        away_name: match.away,
        home_goals: match.hg ?? null,
        away_goals: match.ag ?? null,
        status: match.status || "",
        note: "",
        resolution: home.status === "resolved" && away.status === "resolved" ? "resolved" : "unresolved",
        issues: [...home.issues, ...away.issues],
      });
    }
  }
  return rows;
};

const scorerRows = (competitionId, rows) => (rows || []).map((player, index) => {
  const team = resolveTeam(competitionId, player.team);
  return {
    id: uniqueId("scorer", `${competitionId}-${player.name}`),
    rank: player.rank ?? index + 1,
    player_name: player.name,
    team_id: team.id,
    team_name: team.name,
    nationality: player.nat || "",
    goals: player.goals,
    appearances: player.apps,
    assists: player.assists,
    resolution: team.status,
    issues: team.issues,
  };
});

const cupRoundId = (competitionId, name) => uniqueId("round", `${competitionId}-${name}`);
const cupTiesFor = (competitionId, cup, rounds) => {
  const ties = [];
  const attach = (list, stage, statusOverride) => {
    for (const [index, tie] of (list || []).entries()) {
      const round = rounds.find((r) => r.name.toLowerCase().includes(stage.toLowerCase()))
        || rounds.find((r) => r.name === stage);
      ties.push({
        id: uniqueId("tie", `${competitionId}-${stage}-${tie.home}-${tie.away}`),
        round_id: round?.id || null,
        stage,
        home_name: tie.home,
        away_name: tie.away,
        home_goals: tie.hg ?? null,
        away_goals: tie.ag ?? null,
        status: statusOverride || tie.status || "",
        note: tie.note || "",
        sort_order: index + 1,
      });
    }
  };
  attach(cup.qf, "Quarter-finals");
  attach(cup.sf, "Semi-finals");
  attach(cup.final, "Final");
  return ties;
};

const cupFor = (competitionId, cup) => {
  const rounds = (cup.rounds || []).map((round, index) => ({
    id: cupRoundId(competitionId, round.name),
    name: round.name,
    date_label: round.date || "",
    clubs: round.clubs ?? null,
    status: round.status || "",
    note: round.note || "",
    sort_order: index + 1,
  }));
  return {
    name: cup.name || "",
    sponsor: cup.sponsor || "",
    teams_count: cup.teams ?? null,
    prize: cup.prize || "",
    reward: cup.winner || "",
    rounds,
    ties: cupTiesFor(competitionId, cup, rounds),
    past_editions: (cup.pastEditions || []).map((e, index) => ({
      id: uniqueId("past-edition", `${competitionId}-${e.year}-${index}`),
      year: e.year,
      home_name: e.home,
      away_name: e.away,
      home_goals: e.hg ?? null,
      away_goals: e.ag ?? null,
      note: e.note || "",
    })),
  };
};

const season = (competitionId, label, options = {}) => ({
  id: options.id || seasonId(label),
  label,
  is_current: options.is_current ?? true,
  verified: false,
  captured_at: options.captured_at || "2026-03-22",
  age_group: options.age_group || null,
  source_ids: options.source_ids || ["legacy-import"],
  groups: options.groups || [],
  matches: options.matches || [],
  cup: options.cup || null,
  scorers: options.scorers || [],
  news_ids: options.news_ids || [],
});

const newsIdsFor = (competitionId) => news.filter((n) => n.competition_ids.includes(competitionId)).map((n) => n.id);

const competitions = competitionDefs.map((def) => {
  let seasons = [];
  if (def.id === "thai-league-1") {
    const group = {
      id: "overall", name: "Overall",
      standings: standingsFor(def.id, "overall", legacy.T1),
      validation: validationFor(legacy.T1),
    };
    seasons = [season(def.id, "2025/26", {
      groups: [group],
      matches: [...matchRows(def.id, legacy.RESULTS.t1, "result"), ...matchRows(def.id, legacy.FIXTURES.t1, "fixture")],
      scorers: scorerRows(def.id, legacy.SCORERS.t1),
      news_ids: newsIdsFor(def.id),
    })];
  } else if (def.id === "thai-league-2") {
    const group = {
      id: "overall", name: "Overall",
      standings: standingsFor(def.id, "overall", legacy.T2),
      validation: validationFor(legacy.T2),
    };
    seasons = [season(def.id, "2025/26", {
      groups: [group],
      matches: [...matchRows(def.id, legacy.RESULTS.t2, "result"), ...matchRows(def.id, legacy.FIXTURES.t2, "fixture")],
      scorers: scorerRows(def.id, legacy.SCORERS.t2),
      news_ids: newsIdsFor(def.id),
    })];
  } else if (def.id === "thai-league-3") {
    const groups = legacy.T3G.map((group, index) => {
      const groupId = `group-${index + 1}`;
      return {
        id: groupId,
        name: group.name,
        standings: standingsFor(def.id, groupId, group.teams),
        validation: validationFor(group.teams),
      };
    });
    seasons = [season(def.id, "2025/26", { groups, news_ids: newsIdsFor(def.id) })];
  } else if (def.id === "thai-fa-cup") {
    seasons = [season(def.id, "2025/26", { cup: cupFor(def.id, cups.fac), news_ids: newsIdsFor(def.id) })];
  } else if (def.id === "thai-league-cup") {
    seasons = [season(def.id, "2025/26", { cup: cupFor(def.id, cups.lc), news_ids: newsIdsFor(def.id) })];
  } else if (def.id === "thai-champion-cup") {
    seasons = [season(def.id, "2026", { cup: cupFor(def.id, cups.cc), news_ids: newsIdsFor(def.id) })];
  } else if (def.id.startsWith("thai-school-league-")) {
    const ages = def.id === "thai-school-league-general"
      ? ["U18", "U16", "U14", "U12"]
      : ["U18", "U16", "U14"];
    seasons = ages.map((age, index) => season(def.id, `2026 · ${age}`, {
      id: `2026-${age.toLowerCase()}`,
      age_group: age,
      is_current: index === 0,
      groups: [],
    }));
  } else {
    seasons = [season(def.id, "2026", { age_group: def.age_group, groups: [] })];
  }
  return { ...def, seasons };
});

const legacyCompetitions = [
  {
    id: "legacy-thai-league-4", name: "Thai League 4 (Semi-Pro)", short_name: "T4",
    section: "Legacy Archive", category: "Senior", kind: "league", age_group: null,
    sort_order: 900, active: false, official_name: "Thai League 4 (Semi-Pro)",
    aliases: ["Thai League 4"], group_label: null,
    description: "Preserved legacy reference. Not shown as an active sidebar competition.",
    source_ids: ["legacy-import"],
    seasons: [{
      id: "2026", label: "2026", is_current: false, verified: false, captured_at: "2026-03-22",
      source_ids: ["legacy-import"],
      groups: legacy.T4G.map((group, index) => ({
        id: `legacy-group-${index + 1}`, name: group.name,
        standings: group.teams.map((t, teamIndex) => ({
          team_id: `legacy-t4-${slug(t.name)}`, team_name: t.name, rank: teamIndex + 1,
          played: t.mp, won: t.w, drawn: t.d, lost: t.l, goals_for: t.gf,
          goals_against: t.ga, goal_difference: t.gd, points: t.pts, form: [], zone: "",
        })),
        validation: { status: "no-data", messages: ["Legacy pre-season placeholder; no match data."] },
      })),
      matches: [], cup: null, scorers: [], news_ids: ["news-11"],
    }],
  },
  {
    id: "legacy-thai-league-5", name: "Thai League 5 (Amateur)", short_name: "T5",
    section: "Legacy Archive", category: "Senior", kind: "league", age_group: null,
    sort_order: 910, active: false, official_name: "Thai League 5 (Amateur)",
    aliases: ["Thailand Amateur League"], group_label: null,
    description: "Preserved legacy reference. Not shown as an active sidebar competition.",
    source_ids: ["legacy-import"],
    seasons: [{
      id: "2026", label: "2026", is_current: false, verified: false, captured_at: "2026-03-22",
      source_ids: ["legacy-import"],
      groups: legacy.TAG.map((group, index) => ({
        id: `legacy-group-${index + 1}`, name: group.name,
        standings: group.teams.map((t, teamIndex) => ({
          team_id: `legacy-t5-${slug(t.name)}`, team_name: t.name, rank: teamIndex + 1,
          played: t.mp, won: t.w, drawn: t.d, lost: t.l, goals_for: t.gf,
          goals_against: t.ga, goal_difference: t.gd, points: t.pts, form: [], zone: "",
        })),
        validation: { status: "no-data", messages: ["Legacy pre-season placeholder; no match data."] },
      })),
      matches: [], cup: null, scorers: [], news_ids: ["news-13"],
    }],
  },
];

const nav = [
  {
    title: "National League",
    subtitle: null,
    items: [
      { type: "competition", id: "thai-league-1" },
      { type: "competition", id: "thai-league-2" },
      { type: "competition", id: "thai-league-3" },
      { type: "competition", id: "thai-fa-cup" },
      { type: "competition", id: "thai-league-cup" },
      { type: "competition", id: "thai-champion-cup" },
    ],
  },
  {
    title: "Youth League (FAT & SAT)",
    subtitle: null,
    items: [
      { type: "competition", id: "thai-youth-league-u18", subtitle: "ฟุตบอลเยาวชน สมาคมฯ ลีก 1" },
      { type: "competition", id: "thai-youth-league-u16", subtitle: "ฟุตบอลเยาวชน สมาคมฯ ลีก 2" },
      { type: "competition", id: "thai-youth-league-u14", subtitle: "ฟุตบอลเยาวชน สมาคมฯ ลีก 3" },
      { type: "competition", id: "thai-youth-fa-cup-u17", subtitle: "ฟุตบอลเยาวชน โค้กคัพ Coke Cup" },
      { type: "competition", id: "thai-youth-league-cup-u18", subtitle: "ฟุตบอลเยาวชน แชมป์กีฬา 7HD" },
      { type: "competition", id: "thai-youth-champion-cup-u18", subtitle: "ฟุตบอลเยาวชน จตุรมิตรสามัคคี" },
    ],
  },
  {
    title: "School League (DPE)",
    subtitle: null,
    items: [
      {
        type: "group", label: "Thai School League — Type A",
        subtitle: "(ฟุตบอลนักเรียน กรมพลฯ ประเภท ก)",
        children: [
          { type: "season", id: "thai-school-league-type-a", season: "2026-u18", label: "U18" },
          { type: "season", id: "thai-school-league-type-a", season: "2026-u16", label: "U16" },
          { type: "season", id: "thai-school-league-type-a", season: "2026-u14", label: "U14" },
        ],
      },
      {
        type: "group", label: "Thai School League — Type B",
        subtitle: "(ฟุตบอลนักเรียน กรมพลฯ ประเภท ข)",
        children: [
          { type: "season", id: "thai-school-league-type-b", season: "2026-u18", label: "U18" },
          { type: "season", id: "thai-school-league-type-b", season: "2026-u16", label: "U16" },
          { type: "season", id: "thai-school-league-type-b", season: "2026-u14", label: "U14" },
        ],
      },
      {
        type: "group", label: "Thai School League — General",
        subtitle: "(ฟุตบอลนักเรียน กรมพลฯ ประเภท ทั่วไป)",
        children: [
          { type: "season", id: "thai-school-league-general", season: "2026-u18", label: "U18" },
          { type: "season", id: "thai-school-league-general", season: "2026-u16", label: "U16" },
          { type: "season", id: "thai-school-league-general", season: "2026-u14", label: "U14" },
          { type: "season", id: "thai-school-league-general", season: "2026-u12", label: "U12" },
        ],
      },
      { type: "competition", id: "thai-college-cup-u19", subtitle: "(ฟุตบอลอาชีวะระดับประเทศ)" },
      { type: "competition", id: "thai-university-cup-u23", subtitle: "(ฟุตบอลมหาวิทยาลัยระดับประเทศ)" },
    ],
  },
];

const seed = {
  meta: {
    app: "Thailand Football",
    mode: "local-preview",
    generated_at: new Date().toISOString(),
    legacy_source: REF,
    legacy_sha256: sha256,
    notes: [
      "Imported records preserve the reference values; they are not presented as independently verified.",
      "T4/T5 are preserved as inactive legacy competitions and are not active sidebar entries.",
      "Youth categories are owner-defined organizational labels and have no imported records.",
      "School competitions use Type A, Type B and General; age groups are season-level dimensions.",
    ],
  },
  sources,
  nav,
  competitions,
  news,
  legacy: legacyCompetitions,
};

fs.mkdirSync(path.join(ROOT, "data"), { recursive: true });
fs.writeFileSync(path.join(ROOT, "data", "seed.json"), JSON.stringify(seed, null, 2), "utf8");
fs.writeFileSync(path.join(ROOT, "data", "legacy_t4_t5.json"), JSON.stringify(legacyCompetitions, null, 2), "utf8");
fs.writeFileSync(path.join(ROOT, "data", "legacy_source_manifest.json"), JSON.stringify({
  source_path: REF,
  sha256,
  extracted_at: new Date().toISOString(),
  counts: {
    competitions: competitions.length,
    active_competitions: competitions.filter((c) => c.active).length,
    team_rows: activeTeamRows.length,
    unique_team_names: teamIdByName.size,
    news: news.length,
    results: Object.values(legacy.RESULTS).flat().reduce((n, r) => n + r.matches.length, 0),
    fixtures: Object.values(legacy.FIXTURES).flat().reduce((n, r) => n + r.matches.length, 0),
    scorers: Object.values(legacy.SCORERS).flat().length,
  },
}, null, 2), "utf8");

console.log(JSON.stringify({
  seed: path.join(ROOT, "data", "seed.json"),
  legacy: path.join(ROOT, "data", "legacy_t4_t5.json"),
  competitions: competitions.length,
  active: competitions.filter((c) => c.active).length,
  teams: activeTeamRows.length,
  uniqueTeams: teamIdByName.size,
  news: news.length,
  sha256,
}, null, 2));
