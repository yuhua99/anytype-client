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

- [ ] Add builders or constructors for complex requests:
  - [ ] `SearchRequest`
  - [ ] `CreateObjectRequest`
  - [ ] `UpdateObjectRequest`
  - [ ] property update payloads
- [ ] Move CLI JSON parsing into named parser modules:
  - [ ] raw object parsing in domain-specific parser module
  - [ ] property parsing in `services/property_values.rs`
  - [x] filter parsing in `commands/filter_parsing.rs`
- [ ] Return actionable errors with arg name and example.
- [ ] Avoid silent fallback except explicitly documented compatibility paths.
- [ ] Add tests for each parser module.

Exit criteria:

- [ ] Request construction logic has unit tests and is not duplicated across commands.

---

## Phase 4 — API client quality

- [ ] Make endpoint paths centralized or strongly grouped.
- [ ] Ensure all API methods accept typed request structs and return typed responses.
- [ ] Add request tests using mocked transport or test client.
- [ ] Consider trait boundary for HTTP transport only if tests or multiple transports need it:

```rust
trait Transport {
    async fn request<T, R>(&self, method: Method, path: &str, body: Option<T>) -> Result<R>;
}
```

- [ ] Keep retry/auth/config concerns out of endpoint methods.
- [ ] Add consistent pagination handling.

Exit criteria:

- [ ] API layer is boring HTTP glue, easy to mock, hard to misuse.

---

## Phase 5 — Output/rendering cleanup

- [ ] Define output contract:
  - [ ] json: machine-stable
  - [ ] yaml: machine-stable
  - [ ] table: human-friendly
- [ ] Move all table formatting into output modules.
- [ ] Avoid printing inside services/API.
- [ ] Add output tests for representative objects.
- [ ] Standardize success messages.

Exit criteria:

- [ ] Changing output format does not touch command/service/API logic.

---

## Phase 6 — OpenAPI/schema policy

Pick one path.

### Option A: no committed live snapshot

- [ ] Add script: `scripts/fetch-openapi.sh`.
- [ ] Add ignored dir: `.openapi/`.
- [ ] Document how to refresh schema locally.

### Option B: committed normalized schema

- [ ] Move schema to `schemas/anytype.openapi.json`.
- [ ] Normalize formatting/deterministic order.
- [ ] Use it for one of:
  - [ ] code generation
  - [ ] schema drift tests
  - [ ] docs generation
- [ ] Add CI check for drift or validation.

Exit criteria:

- [ ] No unreferenced generated/live artifacts in repo root.

---

## Phase 7 — Docs and examples as tests

- [ ] Update `skills/anyclient/SKILL.md` examples.
- [ ] Add docs examples for:
  - [ ] search filters typed
  - [ ] search filters legacy raw
  - [ ] object create/update properties
  - [ ] tags
  - [ ] files
- [ ] Add smoke test script for examples where possible.
- [ ] Ensure every documented command still parses.

Exit criteria:

- [ ] Docs cannot silently rot.

---

## Phase 8 — Error handling policy

- [ ] Decide `anyhow` boundary:
  - [ ] OK in CLI/command layer
  - [ ] consider typed errors in parser/service/API layers
- [ ] Add context to every fallible external operation:
  - [ ] config load
  - [ ] HTTP request
  - [ ] JSON parse
  - [ ] file IO
- [ ] Remove silent catch-alls.
- [ ] Standardize error wording.

Exit criteria:

- [ ] User knows what failed, where, and how to fix it.

---

## Phase 9 — Lints and quality gates

- [x] Enable strict clippy in CI. See `.github/workflows/ci.yml`.
- [ ] Evaluate additional lint denies after architecture stabilizes.
- [ ] Fix warnings instead of allowing globally.
- [ ] Consider selected lint denies:
  - [ ] `clippy::unwrap_used` outside tests
  - [ ] `clippy::expect_used` outside tests
  - [ ] `clippy::panic` outside tests
  - [ ] `clippy::large_enum_variant`
  - [ ] `clippy::wildcard_imports`
- [ ] Add `cargo deny` or dependency audit if project needs supply-chain hygiene.

Exit criteria:

- [ ] Main branch always formatted, lint-clean, test-clean.

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
