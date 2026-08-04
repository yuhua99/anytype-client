# AGENTS.md

## Architecture contract

- `src/commands/` must not contain business workflows, complex JSON construction, aggregation, or API response post-processing beyond routing.
- `src/services/` must not render output or parse CLI arguments.
- `src/api/` must not parse CLI args, resolve names, or print.
- `src/models/` must not perform network calls or command orchestration.
- `src/output.rs` must not perform HTTP calls, config reads, or domain resolution.

Do not create catch-all modules or directories; name modules by domain ownership.

## File size and split rules

- Target source files under ~600 LOC; split approaching files by ownership rather than into generic helpers.
- Keep tests cohesive; target ~600 LOC and do not exceed 1000 LOC. Do not split tests artificially to meet the target.

## Testing contract

Keep CLI parse/help smoke, request serialization, core model serde round-trip, and stable JSON/YAML/table output rendering coverage.

## Toolchain and quality gates

Use stable Rust with the `clippy` and `rustfmt` components; `Cargo.toml` uses edition 2024.

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Commit message rules

Use `<type>: <imperative summary>` with: `feat`, `fix`, `refactor`, `docs`, or `chore`.
