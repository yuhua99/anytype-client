# TODO — architecture review findings

## High priority

- [x] 1. Tolerate unknown enum values in API responses.
      `ObjectLayout`, `PropertyFormat`, `IconColor` are closed enums on the
      response path; one unknown value (e.g. layout `set`) fails the whole
      list/search deserialization. Add `Unknown` fallback variants
      (response-side only; CLI input stays strict).
- [x] 2. Fix tag property key/name mismatch (silent tag wipe).
      `resolve_property` resolves name→id, but `get_object_tag_ids` matches by
      `key` and the write uses the raw user string as `key`. Passing a display
      name reads current tags as empty and `--tag-add` replaces/wipes them.
      Resolve to the canonical property key once and use it everywhere.
- [x] 3. Include operation (method + path) in HTTP error messages.
      `decode_response` errors reference "caller context" that no caller adds.
- [x] 4a. Move `objects find` filtering server-side via typed
      `FilterExpression` (tag, property, missing-property); make `--name`
      behave as documented (substring on name).
- [x] 4b. Use `pagination.total` for ungrouped `objects count` instead of
      downloading every object.

## Medium priority

- [x] 5. Print a stderr notice when fuzzy (substring) resolution picks a
      space/property/tag, so destructive commands are not silently redirected.
- [ ] 6. Skip the full `GET /spaces` round-trip when the input resolves
      directly as a space ID.
- [ ] 7. Deduplicate the page-limit constant (commands hardcodes 1000, api has
      `PAGE_LIMIT`); drop redundant comma re-splitting in `load_object_ids`.
- [ ] 8. Replace `unreachable!()` for `Command::Auth` in `run_command` with an
      error.
- [ ] 9. (Deferred) Verify speculative serde aliases (`"ID"`, `"Name"`,
      `"defaultTemplateId"`, …) against the pinned `Anytype-Version` on a live
      API and remove the unneeded ones. Needs live-API evidence; not safe to
      remove blind.

## Testing gaps

- [x] T1. Unit tests for resolution logic (space/property/tag): exact-vs-fuzzy
      precedence, ambiguity errors.
- [x] T2. Tag merge semantics tests (add/remove/idempotency, key≠name case).
- [ ] T3. Wiremock test for the auto-pagination loop (multi-page accumulation
      and stall guard).
- [x] T4. Response-decoding fixtures with unknown layout/format/color values.
- [ ] T5. Tests for count grouping helpers (`count_by_property`,
      `display_property_value`, missing/empty buckets).
- [ ] T6. Config load/save round-trip test, including the `app_key` alias.
