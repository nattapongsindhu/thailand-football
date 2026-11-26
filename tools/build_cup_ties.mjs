#!/usr/bin/env node
/**
 * Build the 2025-26 Thai FA Cup and Thai League Cup rounds/ties in
 * data/seed.json from captured Wikipedia wikitext.
 *
 * Run: node tools/build_cup_ties.mjs
 *
 * Sources (captured raw wikitext, committed for reproducibility):
 *   data/discovery/sources/fa_cup_2025_26.wiki
 *   data/discovery/sources/league_cup_2025_26.wiki
 *
 * The tool also records the Wikipedia team-name aliases on the existing
 * permanent team identities and marks the cup participations as
 * verified_secondary, because both the entry list and the match record come
 * from the same secondary source.
 *
 * It never creates a team, never changes a team_id or display name, and never
 * fabricates a score or date.
 */
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(import.meta.dirname, "..");
const SEED = path.join(ROOT, "data", "seed.json");
const REGISTRY = path.join(ROOT, "data", "team_registry.json");
const CURATED = path.join(ROOT, "data", "discovery", "curated_verified_teams.json");
const SOURCES = path.join(ROOT, "data", "discovery", "sources");
const NOW = "2026-11-26";

const MONTHS = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
];

const competitions = [
  {
    id: "thai-fa-cup",
    wiki: "fa_cup_2025_26.wiki",
    sourceUrl: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_FA_Cup",
    sourceTitle: "2025-26 Thai FA Cup",
    roundNames: {
      "Qualification round": "Qualification Round",
      "First round": "First Round",
      "Second round": "Second Round",
      "Third round": "Third Round",
      "Quarter-finals": "Quarter-finals",
      "Semi-finals": "Semi-finals",
      Final: "Final",
    },
    aliases: {
      "Nonthaburi United": "team-nonthaburi-utd",
      Marines: "team-marines-eureka",
      "Bankhai United": "team-ban-khai-united",
      "Nakhon Ratchasima Mazda": "team-nakhon-ratchasima",
      "Mahasarakham SBT": "team-mahasarakham",
      "Kanchanaburi Power": "team-kanchanaburi",
      "Kasem Bundit University": "team-kasem-bundit-u",
    },
  },
  {
    id: "thai-league-cup",
    wiki: "league_cup_2025_26.wiki",
    sourceUrl: "https://en.wikipedia.org/wiki/2025%E2%80%9326_Thai_League_Cup",
    sourceTitle: "2025-26 Thai League Cup",
    roundNames: {
      "First qualification round": "1st Qualification Round",
      "Second qualification round": "2nd Qualification Round",
      "Qualification play-off round": "Qualification Play-off",
      "First round": "First Round",
      "Second round": "Second Round",
      "Quarter-finals": "Quarter-finals",
      "Semi-finals": "Semi-finals",
      Final: "Final",
    },
    aliases: {
      "Saraburi United": "team-saraburi-utd",
      Marines: "team-marines-eureka",
      "Kasem Bundit University": "team-kasem-bundit-u",
      "Bankhai United": "team-ban-khai-united",
      "Nonthaburi United": "team-nonthaburi-utd",
      "PT Satun": "team-fc-satun",
      "Mahasarakham SBT": "team-mahasarakham",
      "Kanchanaburi Power": "team-kanchanaburi",
      "Nakhon Ratchasima Mazda": "team-nakhon-ratchasima",
    },
  },
];

const norm = (value) =>
  (value || "")
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/\b(f\.?c\.?|football club)\b/g, "")
    .replace(/[^a-z0-9]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();

const slug = (value) =>
  (value || "")
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/&/g, " and ")
    .replace(/['’]/g, "")
    .replace(/[^a-zA-Z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();

function parseRounds(file) {
  const text = fs.readFileSync(file, "utf8");
  const sections = [];
  let current = { name: null, raw: "" };
  for (const line of text.split(/\r?\n/)) {
    const heading = line.match(/^===\s*(.*?)\s*===$/);
    if (heading) {
      current = { name: heading[1], raw: "" };
      sections.push(current);
    } else if (current.name) {
      current.raw += line + "\n";
    }
  }
  const rounds = [];
  for (const section of sections) {
    const blocks = section.raw.split(/\n\{\{football box collapsible/).slice(1);
    if (blocks.length === 0) continue;
    const matches = blocks.map((block) => {
      const body = block.split(/\n\}\}/)[0];
      const fields = {};
      let last = null;
      for (const line of body.split(/\r?\n/)) {
        const m = line.match(/^\|\s*([a-zA-Z0-9_]+)\s*=\s*(.*)$/);
        if (m) {
          last = m[1].toLowerCase();
          fields[last] = m[2];
        } else if (last && line.trim() !== "") {
          fields[last] += " " + line.trim();
        }
      }
      return fields;
    });
    rounds.push({ name: section.name, matches });
  }
  return rounds;
}

function cleanTeam(value = "") {
  const source = value
    .replace(/<ref[\s\S]*?<\/ref>/g, "")
    .replace(/<ref[^>]*\/>/g, "")
    .trim();
  let out = "";
  let i = 0;
  while (i < source.length) {
    if (source.startsWith("[[", i)) {
      const end = source.indexOf("]]", i);
      if (end === -1) {
        out += source.slice(i);
        break;
      }
      const inner = source.slice(i + 2, end);
      const pipe = inner.indexOf("|");
      out += pipe === -1 ? inner : inner.slice(pipe + 1);
      i = end + 2;
    } else {
      out += source[i];
      i += 1;
    }
  }
  return out
    .replace(/'''/g, "")
    .replace(/<[^>]+>/g, "")
    .replace(/{{[^{}]*}}/g, "")
    .replace(/\s*\((?:T[123]|TS|TA)\)\s*$/i, "")
    .replace(/\s+/g, " ")
    .trim();
}

function parseDate(value = "") {
  const m = value.match(/\{\{Start date\|(\d{4})\|(\d{1,2})\|(\d{1,2})/i);
  if (!m) throw new Error(`unparseable date: ${value}`);
  return `${m[1]}-${String(m[2]).padStart(2, "0")}-${String(m[3]).padStart(2, "0")}`;
}

function fmtDate(iso) {
  const [year, month, day] = iso.split("-").map(Number);
  return `${day} ${MONTHS[month - 1]} ${year}`;
}

function dateLabel(dates) {
  const unique = [...new Set(dates)].sort();
  if (unique.length === 0) return "TBD";
  const first = unique[0];
  const last = unique[unique.length - 1];
  if (first === last) return fmtDate(first);
  const [year1, month1, day1] = first.split("-").map(Number);
  const [year2, month2, day2] = last.split("-").map(Number);
  if (year1 === year2 && month1 === month2) {
    return `${day1}–${day2} ${MONTHS[month1 - 1]} ${year1}`;
  }
  if (year1 === year2) {
    return `${day1} ${MONTHS[month1 - 1]} – ${day2} ${MONTHS[month2 - 1]} ${year1}`;
  }
  return `${fmtDate(first)} – ${fmtDate(last)}`;
}

function parseScore(raw = "") {
  const awarded = /awd/i.test(raw);
  const cleaned = raw
    .replace(/<ref[\s\S]*?<\/ref>/g, "")
    .replace(/<ref[^>]*\/>/g, "")
    .replace(/<[^>]+>/g, "")
    .replace(/{{[^{}]*}}/g, "")
    .trim();
  const m = cleaned.match(/(\d+)\s*[-–]\s*(\d+)/);
  if (!m) throw new Error(`unparseable score: ${raw}`);
  return { home: Number(m[1]), away: Number(m[2]), awarded };
}

function parsePenalties(raw = "") {
  const m = (raw || "").match(/(\d+)\s*[-–]\s*(\d+)/);
  return m ? { home: Number(m[1]), away: Number(m[2]) } : null;
}

const seed = JSON.parse(fs.readFileSync(SEED, "utf8"));
const registry = JSON.parse(fs.readFileSync(REGISTRY, "utf8"));
const curated = JSON.parse(fs.readFileSync(CURATED, "utf8"));
const registryTeams = new Map(registry.teams.map((team) => [team.team_id, team]));
const curatedTeams = new Map(curated.teams.map((team) => [team.team_id, team]));

for (const competition of competitions) {
  const comp = seed.competitions.find((entry) => entry.id === competition.id);
  if (!comp) throw new Error(`missing competition ${competition.id}`);
  const season = comp.seasons.find((entry) => entry.id === "2025-26");
  if (!season || !season.cup) throw new Error(`missing cup season for ${competition.id}`);

  const participations = registry.participations.filter(
    (entry) => entry.competition_id === competition.id,
  );
  const teamByParticipation = new Map(
    participations.map((entry) => [entry.team_id, registryTeams.get(entry.team_id)]),
  );
  const participants = new Set(participations.map((entry) => entry.team_id));

  const exact = new Map();
  for (const participation of participations) {
    const team = registryTeams.get(participation.team_id);
    for (const key of [team.display_name, team.english_name, ...(team.aliases || [])]) {
      const normalized = norm(key);
      if (normalized && !exact.has(normalized)) exact.set(normalized, team.team_id);
    }
  }

  const resolve = (wikiName) => {
    const direct = exact.get(norm(wikiName));
    if (direct) return direct;
    const override = competition.aliases[wikiName];
    if (override) return override;
    throw new Error(`${competition.id}: unresolved team name ${wikiName}`);
  };

  const rounds = parseRounds(path.join(SOURCES, competition.wiki));
  const existingNotes = new Map(
    season.cup.ties.map((tie) => [
      `${tie.stage}|${tie.home_name}|${tie.away_name}`,
      tie.note,
    ]),
  );
  const existingRounds = new Map(season.cup.rounds.map((round) => [round.name, round]));

  const usedTeams = new Set();
  const addedAliases = [];
  const newRounds = [];
  const newTies = [];

  for (const [roundIndex, rawRound] of rounds.entries()) {
    const round = {
      name: competition.roundNames[rawRound.name] ?? rawRound.name,
      matches: rawRound.matches,
    };
    const roundId =
      existingRounds.get(round.name)?.id ??
      `round-${competition.id}-${slug(round.name)}`;
    const roundSuffix = roundId.replace(`round-${competition.id}-`, "");
    const dates = round.matches.map((match) => parseDate(match.date));
    const goals = round.matches.reduce((total, match) => {
      const score = parseScore(match.score);
      return total + score.home + score.away;
    }, 0);

    const tierNote = (existingRounds.get(round.name)?.note || "")
      .split("·")
      .map((part) => part.trim())
      .find((part) => /^T[123]\b/.test(part));
    const baseNote = `${round.matches.length} match${round.matches.length === 1 ? "" : "es"} · ${goals} goals`;
    const roundNote = tierNote ? `${baseNote} · ${tierNote}` : baseNote;

    newRounds.push({
      id: roundId,
      name: round.name,
      date_label: dateLabel(dates),
      clubs: round.matches.length * 2,
      status: "done",
      note: roundNote,
      sort_order: roundIndex + 1,
    });

    round.matches.forEach((match, index) => {
      const homeId = resolve(cleanTeam(match.team1));
      const awayId = resolve(cleanTeam(match.team2));
      const home = registryTeams.get(homeId);
      const away = registryTeams.get(awayId);
      const homeSourceName = cleanTeam(match.team1);
      const awaySourceName = cleanTeam(match.team2);
      // Prefer the fuller name: the source sometimes carries the formal club
      // name (PT Prachuap) where the standings feed uses a short label
      // (Prachuap), and vice versa (Port vs Port FC).
      const homeName =
        homeSourceName.length >= home.display_name.length
          ? homeSourceName
          : home.display_name;
      const awayName =
        awaySourceName.length >= away.display_name.length
          ? awaySourceName
          : away.display_name;
      usedTeams.add(homeId);
      usedTeams.add(awayId);

      for (const [wikiKey, teamId] of [
        [cleanTeam(match.team1), homeId],
        [cleanTeam(match.team2), awayId],
      ]) {
        if (!competition.aliases[wikiKey]) continue;
        for (const target of [curatedTeams.get(teamId), registryTeams.get(teamId)]) {
          if (!target) continue;
          target.aliases = target.aliases || [];
          if (!target.aliases.some((alias) => norm(alias) === norm(wikiKey))) {
            target.aliases.push(wikiKey);
            addedAliases.push(`${teamId}: ${wikiKey}`);
          }
        }
      }

      const score = parseScore(match.score);
      const penalties = parsePenalties(match.penaltyscore);
      const extraTime = /^yes$/i.test((match.aet || "").trim());
      const winnerId = penalties
        ? penalties.home > penalties.away
          ? homeId
          : awayId
        : score.home > score.away
          ? homeId
          : awayId;
      const winnerName = winnerId === homeId ? homeName : awayName;
      const isFinal = round.name === "Final";
      const winnerBasis = isFinal
        ? "final"
        : penalties
          ? "penalty shootout"
          : score.awarded
            ? "awarded"
            : extraTime
              ? "extra time"
              : "full time";
      const status = score.awarded
        ? "Awarded"
        : penalties && extraTime
          ? "FT (AET, pens)"
          : penalties
            ? "FT (pens)"
            : extraTime
              ? "FT (AET)"
              : "FT";
      const preservedNote = existingNotes.get(`${round.name}|${home.display_name}|${away.display_name}`) || "";
      const note = score.awarded ? "Awarded result" : preservedNote;

      const tie = {
        id: `tie-${competition.id}-${roundSuffix}-${slug(homeName)}-${slug(awayName)}`,
        round_id: roundId,
        stage: round.name,
        home_name: homeName,
        away_name: awayName,
        home_goals: score.home,
        away_goals: score.away,
        status,
        note,
        sort_order: index + 1,
        winner_name: winnerName,
        winner_basis: winnerBasis,
      };
      if (penalties) {
        tie.home_penalties = penalties.home;
        tie.away_penalties = penalties.away;
      }
      if (extraTime) tie.extra_time = true;
      newTies.push(tie);
    });
  }

  if (newTies.length === 0) throw new Error(`${competition.id}: no ties parsed`);
  const missing = [...participants].filter((teamId) => !usedTeams.has(teamId));
  if (missing.length > 0) {
    throw new Error(
      `${competition.id}: entered clubs without a match: ${missing.join(", ")}`,
    );
  }
  if (usedTeams.size !== participants.size) {
    throw new Error(
      `${competition.id}: mapped ${usedTeams.size} clubs, expected ${participants.size}`,
    );
  }

  const finalRound = newRounds[newRounds.length - 1];
  const finalTie = newTies.find((tie) => tie.round_id === finalRound.id);
  finalRound.note = `Champion: ${finalTie.winner_name} · Runner-up: ${
    finalTie.winner_name === finalTie.home_name ? finalTie.away_name : finalTie.home_name
  }`;

  season.cup.rounds = newRounds;
  season.cup.ties = newTies;

  const note = `One of ${participations.length} clubs entered in the ${competition.sourceTitle}; entry and match results imported from the same Wikipedia page; secondary source, official confirmation still preferred.`;
  for (const participation of participations) {
    participation.verification_status = "verified_secondary";
    participation.source_url = competition.sourceUrl;
    participation.source_title = competition.sourceTitle;
    participation.source_publisher = "Wikipedia";
    participation.notes = note;
    participation.last_seen_at = NOW;
    const curatedParticipation = curated.participations.find(
      (entry) => entry.participation_id === participation.participation_id,
    );
    if (curatedParticipation) {
      curatedParticipation.verification_status = "verified_secondary";
      curatedParticipation.source_url = competition.sourceUrl;
      curatedParticipation.source_title = competition.sourceTitle;
      curatedParticipation.source_publisher = "Wikipedia";
      curatedParticipation.notes = note;
      curatedParticipation.last_seen_at = NOW;
    }
  }

  console.log(
    `${competition.id}: ${newRounds.length} rounds, ${newTies.length} ties, ${participations.length} clubs, ${addedAliases.length} aliases`,
  );
  if (addedAliases.length) console.log(`  aliases: ${addedAliases.join(" | ")}`);
}

fs.writeFileSync(SEED, JSON.stringify(seed, null, 2) + "\n", "utf8");
fs.writeFileSync(REGISTRY, JSON.stringify(registry, null, 2) + "\n", "utf8");
fs.writeFileSync(CURATED, JSON.stringify(curated, null, 2) + "\n", "utf8");
