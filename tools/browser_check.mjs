import { chromium } from "playwright-core";
import fs from "node:fs";
import path from "node:path";

const CHROME = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
const BASE = process.env.TF_BASE || "http://127.0.0.1:8080";
const OUT = path.resolve("logs");
fs.mkdirSync(OUT, { recursive: true });

const results = [];
const failures = [];
const consoleErrors = [];

function check(name, condition, detail = "") {
  const passed = Boolean(condition);
  results.push({ name, passed, detail });
  if (!passed) failures.push({ name, detail });
}

const browser = await chromium.launch({ executablePath: CHROME, headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
const page = await context.newPage();
page.on("console", (message) => {
  if (message.type() === "error") consoleErrors.push(message.text());
});
page.on("pageerror", (error) => consoleErrors.push(`pageerror: ${error.message}`));

await page.goto(`${BASE}/competitions/thai-league-1`, { waitUntil: "networkidle" });
check("desktop title", (await page.title()).includes("Thai League 1"));
check("desktop standings row", (await page.locator("text=Buriram United").count()) > 0);
check("preview badge", (await page.locator("text=Local preview").count()) > 0);
const desktopOverflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
check("desktop no horizontal overflow", desktopOverflow <= 1, `overflow=${desktopOverflow}`);

// Team-first lists.
await page.goto(`${BASE}/competitions/thai-league-1/teams`, { waitUntil: "networkidle" });
check("T1 team list rows", (await page.locator(".teams-table tbody tr").count()) === 16);
check("T1 verified team badges", (await page.locator(".teams-table .badge-verified").count()) === 16);
check("T1 team source link", (await page.locator(".teams-table a").count()) > 0);
await page.goto(`${BASE}/competitions/thai-league-3/teams?group=group-2`, { waitUntil: "networkidle" });
check("T3 group team rows", (await page.locator(".teams-table tbody tr").count()) === 5);
check("T3 unverified badges", (await page.locator(".teams-table .badge-unverified").count()) === 5);
await page.goto(`${BASE}/competitions/thai-youth-fa-cup-u17`, { waitUntil: "networkidle" });
check("Coke Cup default tab is Rounds", (await page.locator(".tab.is-active").innerText()).trim() === "Rounds");
check("Coke Cup rounds", (await page.locator("article.cup-round").count()) === 3);
check("Coke Cup ties", (await page.locator(".cup-tie").count()) === 7);
check(
  "Coke Cup quarter-finals",
  (await page.locator("main").innerText()).includes("Assumption United") &&
    (await page.locator("main").innerText()).includes("Kanthararom School") &&
    (await page.locator("main").innerText()).includes("1 - 0") &&
    (await page.locator("main").innerText()).includes("3 - 1"),
);
check(
  "Coke Cup final result",
  (await page.locator("main").innerText()).includes("Champion: Assumption United") &&
    (await page.locator("main").innerText()).includes("6 - 3"),
);
await page.goto(`${BASE}/competitions/thai-youth-fa-cup-u17/teams`, { waitUntil: "networkidle" });
check("Coke Cup team rows", (await page.locator(".teams-table tbody tr").count()) === 12);
await page.goto(`${BASE}/competitions/thai-youth-league-u18`, { waitUntil: "networkidle" });
check("empty youth default tab is Teams", (await page.locator(".tab.is-active").innerText()).trim() === "Teams");
check("youth team empty state", (await page.locator("text=No verified team list yet").count()) > 0);
check("youth team no fallback", (await page.locator("text=Buriram United").count()) === 0);

// Youth League parents with U18/U16/U14 age-group navigation.
const youthGroups = page.locator("#sidebar details.nav-group", { hasText: "Thai Youth League" });
check("youth league has three expandable parents", (await youthGroups.count()) === 3, `groups=${await youthGroups.count()}`);
const youthSubtitles = await page.locator("#sidebar details.nav-group .nav-sub").allTextContents();
check(
  "youth league Thai labels present",
  ["ลีก 1", "ลีก 2", "ลีก 3"].every((label) => youthSubtitles.some((text) => text.includes(label))),
  youthSubtitles.join(" | "),
);
const youthGroup1 = page.locator("#sidebar details.nav-group", { hasText: "Thai Youth League 1" }).first();
const youthChildLabels = await youthGroup1.locator(".nav-child .nav-label").allTextContents();
check(
  "youth league age children",
  JSON.stringify(youthChildLabels) === JSON.stringify(["U18", "U16", "U14"]),
  youthChildLabels.join(","),
);
await page.goto(`${BASE}/competitions/thai-youth-league-u18?season=2026-u16`, { waitUntil: "networkidle" });
const youthHeading = await page.locator("main h1").innerText();
check("youth league parent heading", youthHeading.includes("Thai Youth League 1"), youthHeading);
check("youth league U16 route selected", (await page.locator("#season-select").inputValue()) === "2026-u16");
check("youth league U16 empty team state", (await page.locator("text=No verified team list yet").count()) > 0);
check("youth league U16 roster unavailable", (await page.locator("text=Roster unavailable").count()) > 0);
check(
  "youth league parent auto-expanded",
  (await youthGroup1.getAttribute("open")) !== null,
  "active parent should be open",
);
await page.reload({ waitUntil: "networkidle" });
check("youth league age group survives refresh", (await page.locator("#season-select").inputValue()) === "2026-u16");
await page.goto(`${BASE}/competitions/thai-youth-league-u14/standings?season=2026-u14`, { waitUntil: "networkidle" });
check("youth league U14 standings empty state", (await page.locator("text=No standings yet").count()) > 0);
check("youth league U14 no senior fallback", (await page.locator("text=Buriram United").count()) === 0);
await page.goto(`${BASE}/competitions/thai-league-1/teams`, { waitUntil: "networkidle" });

// Favorite by stable ID, persisted across reload.
const fav = page.locator('button[data-favorite-id="team-buriram-united"]').first();
await fav.click();
check("favorite aria pressed", (await fav.getAttribute("aria-pressed")) === "true");
const stored = await page.evaluate(() => JSON.parse(localStorage.getItem("tf-favorites") || "[]"));
check("favorite stored by team id", stored.includes("team-buriram-united"), JSON.stringify(stored));
await page.reload({ waitUntil: "networkidle" });
const favAfter = page.locator('button[data-favorite-id="team-buriram-united"]').first();
check("favorite restored", (await favAfter.getAttribute("aria-pressed")) === "true");

// Theme persisted.
await page.locator("#theme-toggle").click();
check("dark theme class", await page.evaluate(() => document.documentElement.classList.contains("theme-dark")));
await page.reload({ waitUntil: "networkidle" });
check("dark theme restored", await page.evaluate(() => document.documentElement.classList.contains("theme-dark")));
await page.locator("#theme-toggle").click();

// Search with HTMX.
await page.locator("#global-search").fill("buriram");
await page.waitForSelector("#search-results .search-result-row", { timeout: 5000 });
const searchRows = await page.locator("#search-results .search-result-row").count();
check("search returns results", searchRows >= 1, `rows=${searchRows}`);
const unitedResult = page.locator("#search-results .search-result-row", { hasText: "Buriram United" }).first();
const searchText = await unitedResult.innerText();
check("search result has context", searchText.includes("Thai League 1") && searchText.includes("Overall"), searchText);
await unitedResult.click();
await page.waitForLoadState("networkidle");
check("search navigates with highlight", page.url().includes("highlight=team-buriram-united"), page.url());
const highlighted = await page.locator("tr.is-highlight").count();
check("search highlights correct row", highlighted === 1, `highlighted=${highlighted}`);

// Regional group page.
await page.goto(`${BASE}/competitions/thai-league-3/standings?group=group-2`, { waitUntil: "networkidle" });
check("T3 group heading", (await page.locator("main").innerText()).includes("Northeastern Region"));
check("T3 arithmetic warning", (await page.locator("text=Arithmetic check failed").count()) > 0);
check("T3 team", (await page.locator("text=Buriram Res.").count()) > 0);

// School League pages, age seasons and Thai subtitles.
await page.goto(`${BASE}/competitions/thai-school-league-type-a?season=2026-u18`, { waitUntil: "networkidle" });
const schoolHeading = await page.locator("main h1").innerText();
check("school league Type A heading", schoolHeading.includes("Thai School League — Type A"), schoolHeading);
check("school league Type A default tab is Teams", (await page.locator(".tab.is-active").innerText()).trim() === "Teams");
check("school league Type A default team rows", (await page.locator(".teams-table tbody tr").count()) === 16);
check("school league Type A current roster badge", (await page.locator("text=Current season roster: 2026").count()) > 0);
check("school league U18 season selected", (await page.locator("#season-select").inputValue()) === "2026-u18");
const schoolSubtitles = await page.locator("#sidebar .nav-sub").allInnerTexts();
check(
  "school league Thai subtitles present",
  schoolSubtitles.some((text) => text.includes("ประเภท ก")) &&
    schoolSubtitles.some((text) => text.includes("ประเภท ข")),
  schoolSubtitles.join(" | "),
);
await page.goto(`${BASE}/competitions/thai-school-league-type-a/standings?season=2026-u18`, { waitUntil: "networkidle" });
check("school league Type A standings empty state", (await page.locator("text=No standings yet").count()) > 0);
await page.goto(`${BASE}/competitions/thai-school-league-general?season=2026-u12`, { waitUntil: "networkidle" });
const schoolThreeHeading = await page.locator("main h1").innerText();
check("school general heading", schoolThreeHeading.includes("General"), schoolThreeHeading);
check("school general default tab is Teams", (await page.locator(".tab.is-active").innerText()).trim() === "Teams");
check("school general empty team state", (await page.locator("text=413 teams entered").count()) > 0);
check("school general U12 season selected", (await page.locator("#season-select").inputValue()) === "2026-u12");
check(
  "school general division Thai subtitle",
  (await page.locator("#sidebar .nav-sub").allInnerTexts()).some((text) => text.includes("ประเภท ทั่วไป")),
);
check("school general unavailable roster badge", (await page.locator("text=Roster unavailable").count()) > 0);

// Latest-verified roster fallback policy.
await page.goto(`${BASE}/competitions/thai-school-league-type-a?season=2026-u14`, { waitUntil: "networkidle" });
check("fallback Type A U14 badge", (await page.locator("text=Latest verified roster: 2024").count()) > 0);
check(
  "fallback Type A U14 notice",
  (await page.locator("text=latest verified roster is shown for reference").count()) > 0,
);
check("fallback Type A U14 team rows", (await page.locator(".teams-table tbody tr").count()) === 16);
check(
  "fallback Type A U14 displayed season",
  (await page.locator("main").innerText()).includes("Displayed season 2024"),
);
await page.goto(`${BASE}/competitions/thai-youth-champion-cup-u18/teams`, { waitUntil: "networkidle" });
check("fallback Youth Champion Cup badge", (await page.locator("text=Latest verified roster: 2025").count()) > 0);
check("fallback Youth Champion Cup team rows", (await page.locator(".teams-table tbody tr").count()) === 4);
await page.goto(`${BASE}/competitions/thai-youth-champion-cup-u18/rounds`, { waitUntil: "networkidle" });
const jaturamitrText = await page.locator("main").innerText();
check("Jaturamitr rounds", (await page.locator("article.cup-round").count()) === 5);
check("Jaturamitr ties", (await page.locator(".cup-tie").count()) === 8);
check("Jaturamitr champion", jaturamitrText.includes("Champion: Debsirin School"));
check("Jaturamitr final score", jaturamitrText.includes("3 - 1"));

await page.goto(`${BASE}/competitions/thai-youth-league-cup-u18/rounds`, { waitUntil: "networkidle" });
const sevenHdText = await page.locator("main").innerText();
check("7HD rounds", (await page.locator("article.cup-round").count()) === 1);
check("7HD quarter-final ties", (await page.locator(".cup-tie").count()) === 4);
check("7HD quarter-final date", sevenHdText.includes("30 Nov 2026"));
check("7HD pairings", sevenHdText.includes("Debsirin School") && sevenHdText.includes("Kanthararom School"));
check("7HD upcoming status", sevenHdText.includes("Upcoming"));
check("7HD no fabricated score", !/\d+ - \d+/.test(sevenHdText.slice(sevenHdText.indexOf("Quarter-finals"))));

await page.goto(`${BASE}/competitions/thai-university-cup-u23/rounds`, { waitUntil: "networkidle" });
const universityCupText = await page.locator("main").innerText();
check("University Cup rounds", (await page.locator("article.cup-round").count()) === 3);
check("University Cup ties", (await page.locator(".cup-tie").count()) === 7);
check("University Cup quarter-final unavailable scores", universityCupText.includes("Score unavailable"));
check("University Cup semi-final penalties", universityCupText.includes("Penalties: 5 - 4"));
check("University Cup final penalties", universityCupText.includes("Penalties: 4 - 3"));
check("University Cup champion", universityCupText.includes("Champion: Thailand National Sports University"));
await page.goto(`${BASE}/competitions/thai-university-cup-u23/teams`, { waitUntil: "networkidle" });
check("University Cup team rows", (await page.locator(".teams-table tbody tr").count()) === 11);
check("U19 absent from U23 page title", !(await page.locator("main h1").innerText()).includes("U19"));

await page.goto(`${BASE}/competitions/thai-youth-fa-cup-u17/teams`, { waitUntil: "networkidle" });
check("partial youth cup badge", (await page.locator("text=Partial list: 2026").count()) > 0);
check(
  "partial youth cup notice",
  (await page.locator("text=full current-season team list has not been published or verified").count()) > 0,
);

// Cup pages.
await page.goto(`${BASE}/competitions/thai-fa-cup/rounds`, { waitUntil: "networkidle" });
const cupText = await page.locator("main").innerText();
check("cup sponsor", cupText.includes("Chang FA Cup"));
check("fa cup rounds", (await page.locator("article.cup-round").count()) === 7);
check("fa cup tie records", (await page.locator(".cup-tie").count()) === 94);
check("fa cup champion", cupText.includes("Champion: Buriram United") && cupText.includes("Runner-up: PT Prachuap"));
check("fa cup final date and score", cupText.includes("31 May 2026") && cupText.includes("1 - 0") && cupText.includes("FT (AET)"));
check("fa cup penalties", cupText.includes("Penalties: 5 - 3"));
check("fa cup awarded result", cupText.includes("Awarded"));
check("fa cup no empty round message", !cupText.includes("No tie records for this round."));
await page.goto(`${BASE}/competitions/thai-league-cup/rounds`, { waitUntil: "networkidle" });
const leagueCupText = await page.locator("main").innerText();
check("league cup rounds", (await page.locator("article.cup-round").count()) === 8);
check("league cup tie records", (await page.locator(".cup-tie").count()) === 79);
check("league cup champion", leagueCupText.includes("Champion: Port FC") && leagueCupText.includes("Runner-up: BG Pathum United"));
check("league cup final date and score", leagueCupText.includes("24 May 2026") && leagueCupText.includes("0 - 1"));
check("league cup penalties", leagueCupText.includes("Penalties:"));
check("league cup no empty round message", !leagueCupText.includes("No tie records for this round."));

// University Cup population is covered above.

check("no browser console errors", consoleErrors.length === 0, consoleErrors.join(" | "));

// Health.
const health = await page.evaluate(async (base) => {
  const response = await fetch(`${base}/healthz`);
  return { status: response.status, body: await response.json() };
}, BASE);
check("health endpoint", health.status === 200 && health.body.status === "ok", JSON.stringify(health));

await page.setViewportSize({ width: 390, height: 900 });
await page.goto(`${BASE}/competitions/thai-league-1`, { waitUntil: "networkidle" });
const mobileOverflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
check("mobile no page overflow", mobileOverflow <= 1, `overflow=${mobileOverflow}`);
const tableOverflow = await page.evaluate(() => {
  const wrap = document.querySelector(".table-wrap");
  return wrap ? wrap.scrollWidth - wrap.clientWidth : -1;
});
check("mobile table scrolls inside wrapper", tableOverflow > 0, `tableOverflow=${tableOverflow}`);
await page.locator("[data-sidebar-toggle]").click();
check("mobile sidebar opens", await page.locator("#sidebar.is-open").count() === 1);
await page.locator("[data-sidebar-close]").click();
check("mobile sidebar closes", await page.locator("#sidebar.is-open").count() === 0);
await page.waitForTimeout(300);

await page.screenshot({ path: path.join(OUT, "mobile-home.png"), fullPage: false });
await page.setViewportSize({ width: 1440, height: 1000 });
await page.goto(`${BASE}/competitions/thai-league-1`, { waitUntil: "networkidle" });
await page.screenshot({ path: path.join(OUT, "desktop-home.png"), fullPage: false });

await browser.close();

const report = { base: BASE, results, failures, consoleErrors };
fs.writeFileSync(path.join(OUT, "browser-check.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
if (failures.length > 0) process.exitCode = 1;
