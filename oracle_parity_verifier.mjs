#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';

const FRONTEND_DIR = '/home/ibrahim/code/pstream/pstream-frontend';
const FRONTEND_RS_DIR = '/home/ibrahim/code/pstream/pstream-frontend-rs';
const TMP_DIR = '/tmp';

console.log('================================================================');
console.log(' Milestone 1 TypeScript Parity Oracle Verification Harness');
console.log('================================================================\n');

// 1. Compile & run Rust exporter
console.log('Step 1: Compiling and running Rust static slice exporter...');
execSync(
  `rustc --edition 2024 \
    -L ${FRONTEND_RS_DIR}/target/debug/deps \
    --extern pstream_frontend_rs=${FRONTEND_RS_DIR}/target/debug/deps/libpstream_frontend_rs.rlib \
    --extern serde_json=${FRONTEND_RS_DIR}/target/debug/deps/libserde_json-d4ba44fbbbee01b4.rlib \
    --extern serde=${FRONTEND_RS_DIR}/target/debug/deps/libserde-097d69548a51c3b5.rlib \
    ${FRONTEND_RS_DIR}/export_rust_data.rs -o ${TMP_DIR}/export_rust_data && ${TMP_DIR}/export_rust_data > ${TMP_DIR}/rust_data.json`,
  { stdio: 'inherit' }
);

const rustData = JSON.parse(fs.readFileSync(`${TMP_DIR}/rust_data.json`, 'utf8'));
console.log('✓ Rust static slices loaded successfully.\n');

// 2. Bundle TypeScript datasets via esbuild
console.log('Step 2: Bundling TypeScript datasets...');
const esbuildBin = path.join(FRONTEND_DIR, 'node_modules/.bin/esbuild');

execSync(`${esbuildBin} ${FRONTEND_DIR}/data/avatars.ts --bundle --format=cjs --outfile=${TMP_DIR}/ts_avatars.cjs`);
execSync(`${esbuildBin} ${FRONTEND_DIR}/data/languages.ts --bundle --format=cjs --outfile=${TMP_DIR}/ts_languages.cjs`);
execSync(`${esbuildBin} ${FRONTEND_DIR}/data/genres.ts --bundle --format=cjs --outfile=${TMP_DIR}/ts_genres.cjs`);
execSync(`${esbuildBin} ${FRONTEND_DIR}/data/pageGenres.ts --bundle --format=cjs --outfile=${TMP_DIR}/ts_pageGenres.cjs`);
execSync(`${esbuildBin} ${FRONTEND_DIR}/hooks/kidsManifestBuilder.ts --bundle --format=cjs --outfile=${TMP_DIR}/ts_kidsManifest.cjs`);

import { createRequire } from 'module';
const require = createRequire(import.meta.url);

const tsAvatars = require(`${TMP_DIR}/ts_avatars.cjs`);
const tsLanguages = require(`${TMP_DIR}/ts_languages.cjs`);
const tsGenres = require(`${TMP_DIR}/ts_genres.cjs`);
const tsPageGenres = require(`${TMP_DIR}/ts_pageGenres.cjs`);
const tsKids = require(`${TMP_DIR}/ts_kidsManifest.cjs`);
console.log('✓ TypeScript datasets bundled and required.\n');

let totalChecks = 0;
let passedChecks = 0;
let failures = [];

function assert(condition, message) {
  totalChecks++;
  if (!condition) {
    failures.push(message);
    console.error(`  ✗ FAIL: ${message}`);
  } else {
    passedChecks++;
  }
}

function assertEq(actual, expected, message) {
  totalChecks++;
  const actualStr = JSON.stringify(actual);
  const expectedStr = JSON.stringify(expected);
  if (actualStr !== expectedStr) {
    const err = `${message} | Expected: ${expectedStr.slice(0, 100)} | Got: ${actualStr.slice(0, 100)}`;
    failures.push(err);
    console.error(`  ✗ FAIL: ${err}`);
  } else {
    passedChecks++;
  }
}

// ============================================================================
// 1. AVATARS VERIFICATION
// ============================================================================
console.log('=== Checking Avatars Parity (avatars.ts vs avatars.rs) ===');

assertEq(rustData.avatar_categories.length, tsAvatars.AVATAR_CATEGORIES.length, 'Avatar categories count');
assertEq(rustData.all_avatars.length, tsAvatars.ALL_AVATARS.length, 'All avatars URL count');
assertEq(rustData.default_avatar, tsAvatars.DEFAULT_AVATAR, 'Default avatar URL');

for (let i = 0; i < tsAvatars.AVATAR_CATEGORIES.length; i++) {
  const tsCat = tsAvatars.AVATAR_CATEGORIES[i];
  const rCat = rustData.avatar_categories[i];
  assertEq(rCat.id, tsCat.id, `Category[${i}] ID matches`);
  assertEq(rCat.name, tsCat.name, `Category[${i}] Name matches`);
  assertEq(rCat.avatars.length, tsCat.avatars.length, `Category[${tsCat.id}] avatar count`);

  for (let j = 0; j < tsCat.avatars.length; j++) {
    const tsAv = tsCat.avatars[j];
    const rAv = rCat.avatars[j];
    assertEq(rAv.name, tsAv.name, `Avatar[${tsCat.id}][${j}] name`);
    assertEq(rAv.url, tsAv.url, `Avatar[${tsCat.id}][${j}] url`);
  }
}

for (let i = 0; i < tsAvatars.ALL_AVATARS.length; i++) {
  assertEq(rustData.all_avatars[i], tsAvatars.ALL_AVATARS[i], `ALL_AVATARS[${i}] exact URL match`);
}

// ============================================================================
// 2. LANGUAGES VERIFICATION
// ============================================================================
console.log('=== Checking Languages Parity (languages.ts vs languages.rs) ===');

assertEq(rustData.display_languages.length, tsLanguages.DISPLAY_LANGUAGES.length, 'DISPLAY_LANGUAGES count');
for (let i = 0; i < tsLanguages.DISPLAY_LANGUAGES.length; i++) {
  assertEq(rustData.display_languages[i].code, tsLanguages.DISPLAY_LANGUAGES[i].code, `DISPLAY_LANGUAGES[${i}] code`);
  assertEq(rustData.display_languages[i].label, tsLanguages.DISPLAY_LANGUAGES[i].label, `DISPLAY_LANGUAGES[${i}] label`);
}

const tsLangLabelsKeys = Object.keys(tsLanguages.LANG_LABELS);
assertEq(rustData.lang_labels.length, tsLangLabelsKeys.length, 'LANG_LABELS count');
const rLabelsMap = Object.fromEntries(rustData.lang_labels);
for (const [code, label] of Object.entries(tsLanguages.LANG_LABELS)) {
  assertEq(rLabelsMap[code], label, `LANG_LABELS[${code}]`);
}

const tsLangToOsKeys = Object.keys(tsLanguages.LANG_TO_OS);
assertEq(rustData.lang_to_os.length, tsLangToOsKeys.length, 'LANG_TO_OS count');
const rOsMap = Object.fromEntries(rustData.lang_to_os);
for (const [code, os] of Object.entries(tsLanguages.LANG_TO_OS)) {
  assertEq(rOsMap[code], os, `LANG_TO_OS[${code}]`);
}

assertEq(rustData.subtitle_languages.length, tsLanguages.SUBTITLE_LANGUAGES.length, 'SUBTITLE_LANGUAGES count');
for (let i = 0; i < tsLanguages.SUBTITLE_LANGUAGES.length; i++) {
  assertEq(rustData.subtitle_languages[i].code, tsLanguages.SUBTITLE_LANGUAGES[i].code, `SUBTITLE_LANGUAGES[${i}] code`);
  assertEq(rustData.subtitle_languages[i].label, tsLanguages.SUBTITLE_LANGUAGES[i].label, `SUBTITLE_LANGUAGES[${i}] label`);
}

// Alphabetization check: compare Rust SUBTITLE_LANGUAGES order with JS localeCompare
console.log('--- Alphabetization Oracle Check ---');
const computedSortedSubtitles = Object.entries(tsLanguages.LANG_LABELS)
  .map(([code, label]) => ({ code, label }))
  .sort((a, b) => a.label.localeCompare(b.label));

for (let i = 0; i < computedSortedSubtitles.length; i++) {
  assertEq(
    rustData.subtitle_languages[i],
    computedSortedSubtitles[i],
    `Alphabetization order at index ${i}`
  );
}

// ============================================================================
// 3. GENRES VERIFICATION
// ============================================================================
console.log('=== Checking Genres Parity (genres.ts vs genres.rs) ===');

const tsGenreIds = Object.keys(tsGenres.GENRES).map(Number);
assertEq(rustData.genres.length, tsGenreIds.length, 'GENRES count');
const rGenresMap = Object.fromEntries(rustData.genres.map(g => [g.id, g.name]));
for (const [idStr, name] of Object.entries(tsGenres.GENRES)) {
  const id = Number(idStr);
  assertEq(rGenresMap[id], name, `GENRES[${id}] name`);
}

const tsAdjIds = Object.keys(tsGenres.ADJACENT_GENRES).map(Number);
assertEq(rustData.adjacent_genres.length, tsAdjIds.length, 'ADJACENT_GENRES count');
const rAdjMap = Object.fromEntries(rustData.adjacent_genres.map(a => [a.id, a.adjacent_ids]));
for (const [idStr, adjList] of Object.entries(tsGenres.ADJACENT_GENRES)) {
  const id = Number(idStr);
  assertEq(rAdjMap[id], adjList, `ADJACENT_GENRES[${id}] list`);
}

assertEq(rustData.micro_genres.length, tsGenres.MICRO_GENRES.length, 'MICRO_GENRES total count (327)');
for (let i = 0; i < tsGenres.MICRO_GENRES.length; i++) {
  const tsM = tsGenres.MICRO_GENRES[i];
  const rM = rustData.micro_genres[i];
  assertEq(rM.name, tsM.name, `MICRO_GENRES[${i}] name`);
  assertEq(rM.genres, tsM.genres, `MICRO_GENRES[${i}] genres`);
  assertEq(rM.media_type, tsM.type, `MICRO_GENRES[${i}] media_type`);
  const tsExtra = tsM.extra !== undefined ? tsM.extra : null;
  assertEq(rM.extra, tsExtra, `MICRO_GENRES[${i}] extra`);
}

// DAY_STREAMS
console.log('--- DAY_STREAMS Parity ---');
assertEq(rustData.day_streams.length, Object.keys(tsGenres.DAY_STREAMS).length, 'DAY_STREAMS count (7)');
const rDayMap = Object.fromEntries(rustData.day_streams.map(d => [d.day, d]));
for (const [day, entry] of Object.entries(tsGenres.DAY_STREAMS)) {
  const rD = rDayMap[day];
  assert(rD !== undefined, `Day ${day} exists in Rust`);
  if (rD) {
    assertEq(rD.name, entry.name, `DAY_STREAMS[${day}] name`);
    assertEq(rD.genres, entry.genres, `DAY_STREAMS[${day}] genres`);
    assertEq(rD.media_type, entry.type, `DAY_STREAMS[${day}] media_type`);
    const tsExtra = entry.extra !== undefined ? entry.extra : null;
    assertEq(rD.extra, tsExtra, `DAY_STREAMS[${day}] extra`);
  }
}

// TIME_STREAMS
console.log('--- TIME_STREAMS Parity ---');
assertEq(Object.keys(rustData.time_streams).length, Object.keys(tsGenres.TIME_STREAMS).length, 'TIME_STREAMS slots count (5)');
for (const [slot, entries] of Object.entries(tsGenres.TIME_STREAMS)) {
  const rEntries = rustData.time_streams[slot];
  assert(rEntries !== undefined, `Time slot ${slot} exists in Rust`);
  if (rEntries) {
    assertEq(rEntries.length, entries.length, `TIME_STREAMS[${slot}] count`);
    for (let i = 0; i < entries.length; i++) {
      assertEq(rEntries[i].name, entries[i].name, `TIME_STREAMS[${slot}][${i}] name`);
      assertEq(rEntries[i].genres, entries[i].genres, `TIME_STREAMS[${slot}][${i}] genres`);
      assertEq(rEntries[i].media_type, entries[i].type, `TIME_STREAMS[${slot}][${i}] media_type`);
      const tsExtra = entries[i].extra !== undefined ? entries[i].extra : null;
      assertEq(rEntries[i].extra, tsExtra, `TIME_STREAMS[${slot}][${i}] extra`);
    }
  }
}

// SEASON_STREAMS
console.log('--- SEASON_STREAMS Parity ---');
assertEq(Object.keys(rustData.season_streams).length, Object.keys(tsGenres.SEASON_STREAMS).length, 'SEASON_STREAMS seasons count (4)');
for (const [season, entries] of Object.entries(tsGenres.SEASON_STREAMS)) {
  const rEntries = rustData.season_streams[season];
  assert(rEntries !== undefined, `Season ${season} exists in Rust`);
  if (rEntries) {
    assertEq(rEntries.length, entries.length, `SEASON_STREAMS[${season}] count`);
    for (let i = 0; i < entries.length; i++) {
      assertEq(rEntries[i].name, entries[i].name, `SEASON_STREAMS[${season}][${i}] name`);
      assertEq(rEntries[i].genres, entries[i].genres, `SEASON_STREAMS[${season}][${i}] genres`);
      assertEq(rEntries[i].media_type, entries[i].type, `SEASON_STREAMS[${season}][${i}] media_type`);
      const tsExtra = entries[i].extra !== undefined ? entries[i].extra : null;
      assertEq(rEntries[i].extra, tsExtra, `SEASON_STREAMS[${season}][${i}] extra`);
    }
  }
}

// HOLIDAY_STREAMS
console.log('--- HOLIDAY_STREAMS Parity ---');
assertEq(Object.keys(rustData.holiday_streams).length, Object.keys(tsGenres.HOLIDAY_STREAMS).length, 'HOLIDAY_STREAMS holidays count (6)');
for (const [holiday, entries] of Object.entries(tsGenres.HOLIDAY_STREAMS)) {
  const rEntries = rustData.holiday_streams[holiday];
  assert(rEntries !== undefined, `Holiday ${holiday} exists in Rust`);
  if (rEntries) {
    assertEq(rEntries.length, entries.length, `HOLIDAY_STREAMS[${holiday}] count`);
    for (let i = 0; i < entries.length; i++) {
      assertEq(rEntries[i].name, entries[i].name, `HOLIDAY_STREAMS[${holiday}][${i}] name`);
      assertEq(rEntries[i].genres, entries[i].genres, `HOLIDAY_STREAMS[${holiday}][${i}] genres`);
      assertEq(rEntries[i].media_type, entries[i].type, `HOLIDAY_STREAMS[${holiday}][${i}] media_type`);
      const tsExtra = entries[i].extra !== undefined ? entries[i].extra : null;
      assertEq(rEntries[i].extra, tsExtra, `HOLIDAY_STREAMS[${holiday}][${i}] extra`);
    }
  }
}

// ============================================================================
// 4. PAGE GENRES VERIFICATION
// ============================================================================
console.log('=== Checking Page Genres Parity (pageGenres.ts vs page_genres.rs) ===');

// MOVIE_GENRES
assertEq(rustData.movie_genres.length, tsPageGenres.MOVIE_GENRES.length, 'MOVIE_GENRES count (27)');
for (let i = 0; i < tsPageGenres.MOVIE_GENRES.length; i++) {
  assertEq(rustData.movie_genres[i].id, tsPageGenres.MOVIE_GENRES[i].id, `MOVIE_GENRES[${i}] id`);
  assertEq(rustData.movie_genres[i].name, tsPageGenres.MOVIE_GENRES[i].name, `MOVIE_GENRES[${i}] name`);
}

// TV_GENRES
assertEq(rustData.tv_genres.length, tsPageGenres.TV_GENRES.length, 'TV_GENRES count (25)');
for (let i = 0; i < tsPageGenres.TV_GENRES.length; i++) {
  assertEq(rustData.tv_genres[i].id, tsPageGenres.TV_GENRES[i].id, `TV_GENRES[${i}] id`);
  assertEq(rustData.tv_genres[i].name, tsPageGenres.TV_GENRES[i].name, `TV_GENRES[${i}] name`);
}

// HOME_GENRE_ID_MAP
const tsMapKeys = Object.keys(tsPageGenres.HOME_GENRE_ID_MAP).map(Number);
assertEq(rustData.home_genre_id_map.length, tsMapKeys.length, 'HOME_GENRE_ID_MAP count (10)');
const rHomeMap = Object.fromEntries(rustData.home_genre_id_map.map(([s, m, t]) => [s, { movie: m, tv: t }]));
for (const [idStr, mapping] of Object.entries(tsPageGenres.HOME_GENRE_ID_MAP)) {
  const id = Number(idStr);
  assertEq(rHomeMap[id], mapping, `HOME_GENRE_ID_MAP[${id}] mapping`);
}

// TV_ONLY & MOVIE_ONLY IDs
const tsTvOnly = [10759, 10762, 10763, 10764, 10765, 10766, 10768, 10015, 9648];
const tsMovieOnly = [12, 36, 53, 10402, 10751, 10013];
assertEq(rustData.tv_only_ids.sort(), tsTvOnly.sort(), 'TV_ONLY_GENRE_IDS set');
assertEq(rustData.movie_only_ids.sort(), tsMovieOnly.sort(), 'MOVIE_ONLY_GENRE_IDS set');

// HOME_MOBILE_GENRES
assertEq(rustData.home_mobile_genres.length, tsPageGenres.HOME_MOBILE_GENRES.length, 'HOME_MOBILE_GENRES count (35)');
for (let i = 0; i < tsPageGenres.HOME_MOBILE_GENRES.length; i++) {
  assertEq(rustData.home_mobile_genres[i].id, tsPageGenres.HOME_MOBILE_GENRES[i].id, `HOME_MOBILE_GENRES[${i}] id`);
  assertEq(rustData.home_mobile_genres[i].name, tsPageGenres.HOME_MOBILE_GENRES[i].name, `HOME_MOBILE_GENRES[${i}] name`);
}

// UNIVERSAL_GENRES
assertEq(rustData.universal_genres.length, tsPageGenres.UNIVERSAL_GENRES.length, 'UNIVERSAL_GENRES count (18)');
for (let i = 0; i < tsPageGenres.UNIVERSAL_GENRES.length; i++) {
  assertEq(rustData.universal_genres[i].id, tsPageGenres.UNIVERSAL_GENRES[i].id, `UNIVERSAL_GENRES[${i}] id`);
  assertEq(rustData.universal_genres[i].name, tsPageGenres.UNIVERSAL_GENRES[i].name, `UNIVERSAL_GENRES[${i}] name`);
}

// KIDS_TV_GENRES & KIDS_MOVIE_GENRES
assertEq(rustData.kids_tv_genres.length, tsKids.KIDS_TV_GENRES.length, 'KIDS_TV_GENRES count (16)');
for (let i = 0; i < tsKids.KIDS_TV_GENRES.length; i++) {
  assertEq(rustData.kids_tv_genres[i].id, tsKids.KIDS_TV_GENRES[i].id, `KIDS_TV_GENRES[${i}] id`);
  assertEq(rustData.kids_tv_genres[i].name, tsKids.KIDS_TV_GENRES[i].name, `KIDS_TV_GENRES[${i}] name`);
}

assertEq(rustData.kids_movie_genres.length, tsKids.KIDS_MOVIE_GENRES.length, 'KIDS_MOVIE_GENRES count (17)');
for (let i = 0; i < tsKids.KIDS_MOVIE_GENRES.length; i++) {
  assertEq(rustData.kids_movie_genres[i].id, tsKids.KIDS_MOVIE_GENRES[i].id, `KIDS_MOVIE_GENRES[${i}] id`);
  assertEq(rustData.kids_movie_genres[i].name, tsKids.KIDS_MOVIE_GENRES[i].name, `KIDS_MOVIE_GENRES[${i}] name`);
}

// Behavioral Oracle Test for resolveGenreId
console.log('--- Behavioral Oracle Test: resolveGenreId ---');
// Test across all possible genres from Movie, TV, Mappings, plus arbitrary unmapped IDs
const testIds = [
  ...rustData.movie_genres.map(g => g.id),
  ...rustData.tv_genres.map(g => g.id),
  ...tsMapKeys,
  1, 9999, 21001, 30000
];

// Run a small rust binary to evaluate resolve_genre_id for these test IDs
const testInputJson = JSON.stringify(testIds);
fs.writeFileSync(`${TMP_DIR}/test_ids.json`, testInputJson);

const rustEvalScript = `
use pstream_frontend_rs::data::page_genres::*;
use pstream_frontend_rs::data::genres::MediaType;
use std::fs;

fn main() {
    let ids_str = fs::read_to_string("${TMP_DIR}/test_ids.json").unwrap();
    let ids: Vec<u32> = serde_json::from_str(&ids_str).unwrap();

    let mut movie_res = Vec::new();
    let mut tv_res = Vec::new();
    for id in ids {
        movie_res.push(resolve_genre_id(MediaType::Movie, id));
        tv_res.push(resolve_genre_id(MediaType::Tv, id));
    }

    let out = serde_json::json!({
        "movie": movie_res,
        "tv": tv_res,
    });
    println!("{}", out);
}
`;
fs.writeFileSync(`${TMP_DIR}/eval_resolve.rs`, rustEvalScript);

execSync(
  `rustc --edition 2024 \
    -L ${FRONTEND_RS_DIR}/target/debug/deps \
    --extern pstream_frontend_rs=${FRONTEND_RS_DIR}/target/debug/deps/libpstream_frontend_rs.rlib \
    --extern serde_json=${FRONTEND_RS_DIR}/target/debug/deps/libserde_json-d4ba44fbbbee01b4.rlib \
    --extern serde=${FRONTEND_RS_DIR}/target/debug/deps/libserde-097d69548a51c3b5.rlib \
    ${TMP_DIR}/eval_resolve.rs -o ${TMP_DIR}/eval_resolve && ${TMP_DIR}/eval_resolve > ${TMP_DIR}/rust_resolve_results.json`
);

const rustResolve = JSON.parse(fs.readFileSync(`${TMP_DIR}/rust_resolve_results.json`, 'utf8'));

for (let i = 0; i < testIds.length; i++) {
  const id = testIds[i];
  const tsMovieVal = tsPageGenres.resolveGenreId('movie', id);
  const tsTvVal = tsPageGenres.resolveGenreId('tv', id);

  assertEq(rustResolve.movie[i], tsMovieVal, `resolveGenreId('movie', ${id})`);
  assertEq(rustResolve.tv[i], tsTvVal, `resolveGenreId('tv', ${id})`);
}

// Summary
console.log('\n================================================================');
console.log(` Oracle Verification Summary`);
console.log('================================================================');
console.log(` Total Assertions Tested: ${totalChecks}`);
console.log(` Passed:                 ${passedChecks}`);
console.log(` Failed:                 ${failures.length}`);

if (failures.length > 0) {
  console.log('\nFAILURES:');
  failures.forEach(f => console.log(' - ' + f));
  process.exit(1);
} else {
  console.log('\nVerdict: 100% DATA FIDELITY & PARITY CONFIRMED.');
  process.exit(0);
}
