# AGENTS.md

## Architecture contract

Keep repository organized by ownership. Do not create catch-all modules or directories.

Source layout:

- `src/cli.rs` owns CLI argument structs and clap validation only.
- `src/commands/` owns thin command orchestration: convert CLI args, call services/API, print output.
- `src/services/` owns multi-step workflows, domain resolution, request construction, and business rules.
- `src/api/` owns HTTP client wiring and endpoint methods only.
- `src/models/` owns serde DTOs, schema enums, request/response structs, and small model constructors.
- `src/output.rs` or `src/output/` owns rendering only: table/json/yaml formatting.
- `src/config.rs` owns config paths, load, save, and config shape.
- `tests/` owns behavior, CLI parsing, serde, request serialization, and compatibility tests.
- `docs/` owns human-facing project docs and command inventories.

Boundary rules:

- Commands should not contain business workflows, complex JSON construction, aggregation logic, or API response post-processing beyond trivial routing.
- Services should not render output or know table/json/yaml formatting.
- API methods should not parse CLI args, resolve names, or print.
- Models should not perform network calls or command orchestration.
- Output code should not perform HTTP calls, config reads, or domain resolution.

## No junk drawers

Do not create generic dumping grounds unless ownership is explicit and documented.

Avoid names like:

- `utils.rs`
- `helpers.rs`
- `common.rs` for unrelated things
- `misc.rs`
- `shared.rs`
- `new_*.rs`
- `*_v2.rs`
- `parts/`

Prefer domain names like:

- `tag_resolution.rs`
- `property_values.rs`
- `request_builders.rs`
- `pagination.rs`
- `object_search.rs`
- `filter_parsing.rs`

Names should encode ownership, not implementation history.

## File size and split rules

- Source files should target under ~600 LOC.
- If a source file approaches ~600 LOC, split by ownership before adding more logic.
- Test files should stay cohesive; ~600 LOC is target, ~1000 LOC is hard ceiling.
- Do not split tests into artificial buckets just to satisfy a number.
- Prefer extracting real lifecycle/domain modules over helper dumping grounds.

## Rust module and API surface rules

- Prefer `pub(crate)` for internal APIs.
- Use `pub` only when external crate users or integration tests need it.
- Avoid broad `pub use *` unless it is an intentional public crate API.
- `mod.rs` may declare modules, but should not hide accidental public API growth.
- Public types should be intentional, documented by usage, and covered by tests.
- Remove or wire unused public types rather than leaving dead API surface.

## Type system rules

Use Rust types to make invalid states hard to represent.

- Prefer enums over constrained strings.
- Prefer newtypes for IDs when they clarify boundaries.
- Keep `Option<T>` for real optionality, not as a shortcut for unclear state.
- Avoid silent catch-alls like `_ => default` for user/API input.
- Let clap validate CLI enums where possible.

## JSON and serde rules

- `serde_json::Value` is allowed at explicit raw JSON compatibility boundaries.
- Convert raw JSON into typed models as soon as practical.
- If raw JSON must pass through unchanged, name that path explicitly, e.g. `Raw` or `LegacyRaw`.
- Add `#[serde(deny_unknown_fields)]` where schema strictness matters.
- Be careful with `#[serde(untagged)]`; add tests for variant dispatch, ambiguity, unknown fields, and round trips.
- Request body JSON shape must be covered by serialization tests.

## Error handling rules

- Errors should say what failed, where, and how to fix it when possible.
- Do not silently fallback unless compatibility requires it and tests document it.
- Add context to JSON parsing, config IO, file IO, and HTTP boundary failures.
- `anyhow` is acceptable in CLI/command/service layers; keep lower-level errors actionable.

## Testing contract

Before major refactors, add or keep tests for current behavior.

Required test classes:

- CLI parse/help smoke tests.
- Request body serialization tests.
- Serde round-trip tests for core models.
- Legacy compatibility tests for documented raw JSON escape hatches.
- Output rendering tests for stable json/yaml/table behavior.

Quality gates must pass before commit:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## OpenAPI and generated artifact rules

- Do not commit unreferenced live API snapshots in repo root.
- If schema is committed, place it under `schemas/` and consume it via generation, validation, docs, or drift checks.
- If schema is fetched locally, place it in ignored working directories such as `.openapi/`.
- Generated files must have clear source, refresh command, and ownership.

## Commit message rules

Use `<type>: <imperative summary>`.

Allowed types: `feat`, `fix`, `refactor`, `docs`, `chore`.

Examples:

```text
feat: add typed search sort options
fix: preserve legacy raw search filters
refactor: move search workflow to service layer
docs: document CLI command baseline
chore: add CI quality gates
```

Keep commits focused. Avoid vague messages like `update`, `cleanup`, or `wip`.

## Refactor rules

- Make small commits with one architectural move each.
- Update `todo.md` when completing planned refactor tasks.
- Do not mix formatting-only changes with logic changes unless quality gate requires it.
- Preserve behavior first; change architecture second.
- If behavior changes, add tests that explain new contract.
