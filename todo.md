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
  output/             # renderers
  config.rs
  error.rs            # optional typed app errors
```

Tasks:

- [x] Add `src/services/mod.rs`.
- [ ] Move search workflow into `services/search.rs`.
- [ ] Move object workflows into `services/objects.rs`.
- [ ] Move property/tag resolution into service helpers.
- [ ] Keep `commands/*` limited to:
  - [ ] args in
  - [ ] call service/api
  - [ ] print result
- [ ] Ban HTTP calls from output layer.
- [ ] Ban rendering from service/API layers.

Exit criteria:

- [ ] No command file contains complex JSON building, aggregation, or multi-step domain logic.

---

## Phase 2 — Type system cleanup

- [ ] Replace stringly enums with real enums:
  - [ ] sort property
  - [ ] sort direction
  - [ ] property format
  - [ ] icon color
  - [ ] object layout
  - [ ] any repeated constrained string
- [ ] Introduce newtypes for IDs if useful:
  - [ ] `SpaceId`
  - [ ] `ObjectId`
  - [ ] `PropertyId`
  - [ ] `TagId`
  - [ ] `FileId`
- [ ] Use `serde_json::Value` only at explicit raw JSON boundaries.
- [ ] Add `deny_unknown_fields` where schema strictness is desired.
- [ ] Audit all `pub`; downgrade to `pub(crate)` unless external API needs it.
- [ ] Remove dead public types or wire them into usage.

Exit criteria:

- [ ] Invalid states become unrepresentable where practical.

---

## Phase 3 — Request builders and parsing boundaries

- [ ] Add builders or constructors for complex requests:
  - [ ] `SearchRequest`
  - [ ] `CreateObjectRequest`
  - [ ] `UpdateObjectRequest`
  - [ ] property update payloads
- [ ] Centralize CLI JSON parsing helpers:
  - [ ] raw object parsing
  - [ ] property parsing
  - [ ] filter parsing
- [ ] Return actionable errors with arg name and example.
- [ ] Avoid silent fallback except explicitly documented compatibility paths.
- [ ] Add tests for each parser helper.

Exit criteria:

- [ ] Request construction logic has unit tests and is not duplicated across commands.

---

## Phase 4 — API client quality

- [ ] Make endpoint paths centralized or strongly grouped.
- [ ] Ensure all API methods accept typed request structs and return typed responses.
- [ ] Add request tests using mocked transport or test client.
- [ ] Consider trait boundary for HTTP transport:

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

- [ ] Enable strict clippy in CI.
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
