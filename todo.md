# Refactor TODO: code quality / architecture push to 10/10

Goal: make this repo boring, typed, testable, maintainable.

Core principle:

```text
CLI parses input only.
Commands orchestrate only.
Services own workflows.
API layer owns HTTP only.
Models own schema only.
Output layer owns rendering only.
Tests lock behavior before refactor.
```

---

## Phase 0 — Baseline and safety net

- [x] Record current supported CLI commands and examples. See `docs/cli-commands.md`.
- [x] Add smoke tests for top-level CLI parse/help. See `tests/cli_smoke.rs`.
- [x] Add request-body serialization tests for every API endpoint with JSON request bodies. See `tests/request_serialization.rs`.
- [x] Add serde round-trip tests for core models. See `tests/model_serde.rs`.
- [x] Add legacy compatibility tests for raw JSON inputs. See `tests/legacy_compatibility.rs`.
- [x] Add golden/snapshot tests for table/json/yaml output where stable. See `tests/output_rendering.rs`.
- [x] Add CI command list. See `.github/workflows/ci.yml`:
  - [x] `cargo fmt --check`
  - [x] `cargo clippy --all-targets -- -D warnings`
  - [x] `cargo test`
- [x] Decide MSRV or document Rust toolchain policy. See `rust-toolchain.toml` and `docs/development.md`.

Exit criteria:

- [x] Refactor can begin with tests guarding current behavior.

---

## Phase 1 — Architecture boundaries

Target layout:

```text
src/
  cli.rs              # clap structs only
  commands/           # thin orchestration
  services/           # business workflows
  api/                # HTTP client + endpoint methods
  models/             # request/response DTOs
  output.rs or output/ # renderers
  config.rs
  error.rs            # optional typed app errors
```

Tasks:

- [x] Add `src/services/mod.rs`.
- [x] Move search workflow into `services/search.rs`.
- [x] Move object workflows into `services/objects.rs`:
  - [x] Move `find` and `count` workflows.
  - [x] Split count aggregation into `services/objects/counts.rs`.
  - [x] Move tag ID merge workflow.
  - [x] Move bulk update workflow:
    - [x] Move target object ID loading.
    - [x] Move per-object update planning/execution.
  - [x] Move create/update request construction.
- [x] Move space/property/tag resolution into domain modules:
  - [x] `services/space_resolution.rs`
  - [x] `services/property_resolution.rs`
  - [x] `services/tag_resolution.rs`
- [ ] Keep `commands/*` limited to:
  - [ ] args in
  - [ ] call service/api
  - [ ] print result
- [x] Ban HTTP calls from output layer.
- [x] Ban rendering from service/API layers.

Exit criteria:

- [ ] No command file contains complex JSON building, aggregation, or multi-step domain logic.

---

## Phase 2 — Type system cleanup

- [ ] Replace stringly enums with real enums:
  - [x] sort property
  - [x] sort direction
  - [x] property format
  - [x] icon color
  - [x] object layout
  - [x] any repeated constrained string:
    - [x] object body format
- [x] Introduce newtypes for IDs if useful:
  - [x] Deferred `SpaceId`.
  - [x] Deferred `ObjectId`.
  - [x] Deferred `PropertyId`.
  - [x] Deferred `TagId`.
  - [x] Deferred `FileId`.
  - Decision: keep IDs as `String` for now; current API/CLI passes opaque IDs and newtypes would add wrapper churn before clear misuse patterns.
- [x] Use `serde_json::Value` only at explicit raw JSON boundaries.
- [x] Add `deny_unknown_fields` where schema strictness is desired.
- [ ] Audit all `pub`; downgrade to `pub(crate)` unless external API needs it:
  - [x] Downgrade internal root modules (`commands`, `config`, `services`).
  - [x] Downgrade service module item visibility.
  - [x] Audit API/model item-level visibility; keep `api`, `models`, `cli`, and `output` public as crate API surface.
- [x] Remove dead public types or wire them into usage; remaining public API endpoints/models are intentional crate surface.

Exit criteria:

- [x] Invalid states become unrepresentable where practical.

---

## Phase 3 — Request builders and parsing boundaries

- [x] Add builders or constructors for complex requests:
  - [x] `SearchRequest`
  - [x] `CreateObjectRequest`
  - [x] `UpdateObjectRequest`
  - [x] property update payloads
- [x] Move CLI JSON parsing into named parser modules:
  - [x] raw object parsing in `commands/property_values.rs`
  - [x] property parsing in `commands/property_values.rs`
  - [x] filter parsing in `commands/filter_parsing.rs`
- [x] Return actionable errors with arg name and example.
- [x] Avoid silent fallback except explicitly documented compatibility paths.
- [x] Add tests for each parser module.

Exit criteria:

- [x] Request construction logic has unit tests and is not duplicated across commands.

---

## Phase 4 — API client quality

- [x] Make endpoint paths centralized or strongly grouped.
  - [x] Initial narrow slice: object and search endpoint paths centralized as private helpers in `src/api/mod.rs`.
  - [x] Broadened to types domain (list/create/get/update/delete + templates paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to tags domain (list/get/create/update/delete paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to properties domain (list/create/get/update/delete paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to files domain (upload, download with width, delete with skip_bin paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to lists domain (views, view objects, add/remove objects paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to members domain (list/get paths) using private helpers in `src/api/mod.rs`.
  - [x] Broadened to spaces (collection/detail) and auth (challenges/api-keys) paths using private helpers in `src/api/mod.rs`.
- [x] Ensure all API methods accept typed request structs and return typed responses.
- [x] Add request tests using mocked transport or test client.
  - [x] Initial unit tests for path helpers in `src/api/mod.rs`.
  - [x] Added `tests/api_http_requests.rs` using `wiremock` dev-dep: exercises GET paginated (objects_page) and POST typed body (search_page) with full request verification (path, query, headers including Anytype-Version + Bearer, JSON body) and response deserialization. No production code changes needed (AnytypeClient::new accepts arbitrary base_url).
- [x] Consider trait boundary for HTTP transport only if tests or multiple transports need it.
  Decision: no `Transport` trait introduced today; concrete `AnytypeClient` (owning reqwest + auth/config in `client.rs`) suffices. Existing tests do not justify the refactor yet.

```rust
trait Transport {
    async fn request<T, R>(&self, method: Method, path: &str, body: Option<T>) -> Result<R>;
}
```

- [x] Keep retry/auth/config concerns out of endpoint methods.
  Verified: all auth (bearer, Anytype-Version), base_url, http client, timeout live only in `client.rs`. Endpoint modules (`*.rs`) are pure path + typed body glue calling client methods.
- [x] Add consistent pagination handling.
  Verified: every paginated endpoint uses `*_page(..., Option<PageOptions>)` convenience wrapper + delegates to `self.request_data(..., page)`; `page_path` + `request_paginated` logic centralized in `client.rs`.

Exit criteria:

- [x] API layer is boring HTTP glue, easy to mock, hard to misuse.
  Wiremock-based request tests in `tests/api_http_requests.rs` + architecture audit (client.rs owns all HTTP/auth/config/retry/pagination; endpoints are thin glue) provide practical coverage. No Transport trait added (not justified by current test needs).

---

## Phase 5 — Output/rendering cleanup

- [x] Define output contract:
  - [x] json: machine-stable (pretty JSON via serde_json)
  - [x] yaml: machine-stable (via serde_yaml)
  - [x] table: human-friendly (tabled + sharp style)
  Documented in `src/output.rs` module comment.
- [x] Move all table formatting into output modules.
  Verified: `Table::new` + `Style` only in `src/output.rs`; models use only `#[derive(Tabled)]` + `#[tabled(skip)]` attrs.
- [x] Avoid printing inside services/API.
  Verified: zero `println!`/`print_*`/`output::` in `src/{services,api}/**`; confined to thin `src/commands/*` orchestration.
- [x] Add output tests for representative objects.
  Added in `tests/output_rendering.rs`: exact-string json for `Space` + table for `Property` using real model instances (minimal construction, stable output).
- [x] Standardize success messages.
  Introduced `print_success`/`eprint_status` helpers in `src/output.rs` (thin, avoid over-engineering); converted clear non-data success/status messages (auth flows, collection add/remove, file download/delete, type delete, bulk update summaries). Raw data paths (ids-only, names-only, markdown export) and interactive prompts left as direct `println!` per scope. Preserves stdout/stderr semantics.

Exit criteria:

- [x] Changing output format does not touch command/service/API logic.
  Count total/grouped rendering moved from `commands/objects.rs` (local `print_counts` + inline match) into `output.rs` helpers (`print_count_total`, `print_grouped_counts`). Commands now only dispatch to output fns after service call. Services/API have no rendering. Raw data compatibility paths (ids-only, names-only, markdown export) documented as intentional exceptions (for scripting/pipeability); they bypass normal output formatters by design.

---

## Phase 6 — OpenAPI/schema policy

- [x] Chose policy: no committed live snapshot (follows AGENTS.md: "Do not commit unreferenced live API snapshots in repo root"). No OpenAPI schema, generated artifacts, scripts, or `.openapi/` present (prior removal of root snapshot; searches confirm clean state, no references in code/docs).
- Future schema (if any) would go under `schemas/` only with documented consumption + ownership per AGENTS.md. No generation workflow invented here.

Exit criteria:

- [x] No unreferenced generated/live artifacts in repo root.
  Verified by filesystem + rg inspection: none present.

---

## Phase 7 — Docs and examples as tests

- [x] Update `skills/anyclient/SKILL.md` examples.
  Added typed filter example (complements legacy raw); property values section already covered multi_select etc.
- [x] Add docs examples for:
  - [x] search filters typed (already present + expanded)
  - [x] search filters legacy raw (added concrete example)
  - [x] object create/update properties (added concrete --property examples matching tests)
  - [x] tags (added create example)
  - [x] files (added upload example)
  All examples in `docs/cli-commands.md` now align with parse tests in `tests/cli_smoke.rs`.
- [x] Add smoke test script for examples where possible.
  `tests/cli_smoke.rs` (extended with parse tests for documented examples) serves as the canonical no-network smoke harness for the CLI examples. No separate shell script added (avoids new artifacts/junk per AGENTS.md).
- [x] Ensure every documented command still parses.
  Added parse tests in `tests/cli_smoke.rs` exercising documented examples from `docs/cli-commands.md` and `skills/anyclient/SKILL.md`:
  - search with typed filters (operator/conditions)
  - search with legacy raw filters
  - objects create with repeatable --property JSON (incl multi_select)
  - objects update with --property + --tag-property/--tag-add
  (covers search filters typed/legacy, object properties, tags; no network required; validates clap surface).

Exit criteria:

- [x] Docs cannot silently rot.
  Docs examples (in `docs/cli-commands.md` and `skills/anyclient/SKILL.md`) aligned to parse tests in `tests/cli_smoke.rs`; `cargo test` (cli_smoke suite) covers the documented command surfaces (typed/legacy filters, object properties, tags, etc.) to prevent silent CLI rot.

---

## Phase 8 — Error handling policy

- [x] Decide `anyhow` boundary:
  - [x] OK in CLI/command layer (current usage + AGENTS.md rule).
  - [x] consider typed errors in parser/service/API layers (deferred; not justified — anyhow + targeted context suffices today; lower layers keep errors actionable via `.context()` / custom anyhow messages).
- [ ] Add context to every fallible external operation:
  - [x] config load (with_context on read/parse/create/write in `src/config.rs`).
  - [x] HTTP request (enriched: send failures and non-success responses now include method+path via `op` context strings + `.with_context` on send/text/bytes in client.rs; decode chains to caller context).
  - [x] JSON parse (parsers in `commands/{filter_parsing,property_values}.rs` include arg name + examples + schema errors).
  - [x] file IO (config save/load paths use context).
- [x] Remove silent catch-alls.
  Fixed in `space_resolution.rs`: unresolved name no longer silently passes through as ID (now explicit "space not found" error with guidance).
- [x] Standardize error wording.
  Verified: parsers, resolvers, config, auth, lib, and HTTP boundaries (post-enrichment) follow consistent "what failed / where / how to fix" style with arg names, examples, or operation details (e.g. "{op} failed with status...", "--foo must be...", "use exact ID or name"). Minor CLI flag messages are terse but functional/low-risk. Matches AGENTS.md rule.

Exit criteria:

- [x] User knows what failed, where, and how to fix it.
  (Parsers/resolvers/auth/config/file IO + enriched HTTP errors + removal of the one silent fallback now provide clear actionable messages for all external boundaries. Minor internal flag checks remain terse but do not affect the primary user experience.)

---

## Phase 9 — Lints and quality gates

- [x] Enable strict clippy in CI. See `.github/workflows/ci.yml`.
- [x] Evaluate additional lint denies after architecture stabilizes.
  Decision: CI `clippy --all-targets -- -D warnings` + default lints sufficient; no additional denies (unwrap_used etc.) or `cargo deny` added to avoid churn. Lint policy documented in AGENTS.md.
- [ ] Fix warnings instead of allowing globally.
- [ ] Consider selected lint denies:
  - [ ] `clippy::unwrap_used` outside tests
  - [ ] `clippy::expect_used` outside tests
  - [ ] `clippy::panic` outside tests
  - [ ] `clippy::large_enum_variant`
  - [ ] `clippy::wildcard_imports`
- [ ] Add `cargo deny` or dependency audit if project needs supply-chain hygiene.

Exit criteria:

- [x] Main branch always formatted, lint-clean, test-clean.
  Enforced by CI on every push/PR (see `.github/workflows/ci.yml` and AGENTS.md). Local pre-commit: same three commands.

---

## Phase 10 — Final architecture review

Review checklist:

- [ ] Can new endpoint be added without touching unrelated modules?
- [ ] Can request JSON be tested without network?
- [ ] Can command behavior be tested without live Anytype?
- [ ] Are invalid CLI values rejected by clap/types?
- [ ] Are raw JSON escape hatches explicit?
- [ ] Are docs/examples current?
- [ ] Are public APIs intentional?
- [ ] Are generated artifacts controlled?
- [ ] Is each module boring and narrow?

Exit criteria:

- [ ] Code quality: 9+/10.
- [ ] Architecture: 9+/10.
- [ ] Remaining tradeoffs documented.

---

## Suggested execution order

1. Tests and CI gates.
2. Service layer extraction.
3. Type cleanup/newtypes.
4. Request builders/parsers.
5. API/output cleanup.
6. OpenAPI policy.
7. Docs-as-tests.
8. Final lint hardening.

Do not start major movement before Phase 0 passes.
