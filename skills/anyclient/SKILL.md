---
name: anyclient
description: Use anyclient CLI to interact with Anytype. Use when the user wants to read, create, update, delete, or search Anytype resources — spaces, objects, types, properties, tags, lists, members, files.
---

# anyclient

CLI for Anytype.

**Always pass `-o json`** for parseable output.

## Quick reference

### Search

| Command | Notes |
|---|---|
| `anyclient search [-o json]` | `--query Q`, `--types T`, `--sort created_date|last_modified_date|last_opened_date|name`, `--direction asc|desc`, `--filters JSON`, `--space ID`, `--limit N`, `--offset N` |

### Spaces

| Command | Notes |
|---|---|
| `anyclient spaces list [-o json]` | |
| `anyclient spaces get SPACE [-o json]` | |
| `anyclient spaces create --name NAME [--description DESC] [-o json]` | |
| `anyclient spaces update SPACE [--name NAME] [--description DESC] [-o json]` | |

### Objects

| Command | Notes |
|---|---|
| `anyclient objects list SPACE [-o json]` | |
| `anyclient objects get SPACE OBJ_ID [-o json]` | `--format md` (default) |
| `anyclient objects export SPACE OBJ_ID [-o json]` | `--format md`. Returns markdown body. |
| `anyclient objects create SPACE --name NAME [-o json]` | `--type TYPE` (default `page`), `--body BODY`, `--icon-emoji E`, `--template T`, `--property JSON` (repeatable) |
| `anyclient objects update SPACE OBJ_ID [-o json]` | `--name`, `--type`, `--markdown`, `--icon-emoji`, `--icon-file ID`, `--icon-name N`, `--icon-color C`, `--clear-icon`, `--property JSON` |
| `anyclient objects delete SPACE OBJ_ID [-o json]` | |

#### Property values in create/update

`--property` is repeatable. Each value is a JSON object:

```bash
--property '{"id":"<prop-id>","key":"status","text":"done"}'
--property '{"key":"tags","multi_select":["<tag-id-1>","<tag-id-2>"]}'
```

Or `--properties-json` with a JSON array of all properties at once.

Icon colors: `grey yellow orange red pink purple blue ice teal lime`

### Types

| Command | Notes |
|---|---|
| `anyclient types list SPACE [-o json]` | |
| `anyclient types get SPACE TYPE_ID [-o json]` | |
| `anyclient types create SPACE --name NAME --plural-name PNAME --layout LAYOUT [-o json]` | Layout: `basic`, `profile`, `action`, `note`. Also `--key`, `--icon-emoji`, `--property` |
| `anyclient types update SPACE TYPE_ID [-o json]` | `--name`, `--plural-name`, `--layout`, `--key`, `--icon-emoji`, `--clear-icon`, `--property` |
| `anyclient types delete SPACE TYPE_ID [-o json]` | |
| `anyclient types templates SPACE TYPE_ID [-o json]` | |
| `anyclient types template-get SPACE TYPE_ID TEMPLATE_ID [-o json]` | |

### Properties

| Command | Notes |
|---|---|
| `anyclient properties list SPACE [-o json]` | |
| `anyclient properties get SPACE PROP_ID [-o json]` | |
| `anyclient properties create SPACE --name NAME --format FMT [-o json]` | Format: `text number select multi_select date files checkbox url email phone objects`. Also `--key`, `--tag` |
| `anyclient properties update SPACE PROP_ID --name NAME [-o json]` | Also `--key` |
| `anyclient properties delete SPACE PROP_ID [-o json]` | |

### Tags

| Command | Notes |
|---|---|
| `anyclient tags list SPACE PROP_ID [-o json]` | |
| `anyclient tags get SPACE PROP_ID TAG_ID [-o json]` | |
| `anyclient tags create SPACE PROP_ID --name NAME --color COLOR [-o json]` | Colors: `grey yellow orange red pink purple blue ice teal lime`. Also `--key` |
| `anyclient tags update SPACE PROP_ID TAG_ID [-o json]` | `--name`, `--color`, `--key` |
| `anyclient tags delete SPACE PROP_ID TAG_ID [-o json]` | |

### Collections


| Command | Notes |
|---|---|
| `anyclient collections views SPACE COLLECTION_ID [-o json]` | |
| `anyclient collections objects SPACE COLLECTION_ID VIEW_ID [-o json]` | |
| `anyclient collections add SPACE COLLECTION_ID OBJ_ID [OBJ_ID...] [-o json]` | |
| `anyclient collections remove SPACE COLLECTION_ID OBJ_ID [-o json]` | |

### Members

| Command | Notes |
|---|---|
| `anyclient members list SPACE [-o json]` | |
| `anyclient members get SPACE MEMBER_ID [-o json]` | |

### Files

| Command | Notes |
|---|---|
| `anyclient files upload SPACE PATH [-o json]` | Returns file ID |
| `anyclient files download SPACE FILE_ID --output PATH [-o json]` | `--width N` for image resize, `--force` overwrite |
| `anyclient files delete SPACE FILE_ID [-o json]` | `--skip-bin` |

## Common patterns

### Create a task with properties

```bash
anyclient objects create abc123 --name "Research AI" \
  --type task \
  --property '{"key":"status","select":"<tag-id>"}' \
  -o json
```

### Search with filters

Legacy raw:
```bash
anyclient search --space abc123 --filters '{"type":"and","filters":[{"key":"type","condition":"equal","value":"task"}]}' -o json
```

Typed (preferred for new usage):
```bash
anyclient search --space abc123 --filters '{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}' -o json
```

### Upload then attach file

```bash
FILE_ID=$(anyclient files upload abc123 photo.png -o json | jq -r '.id')
anyclient objects update abc123 obj456 \
  --property "{\"key\":\"cover\",\"files\":[\"$FILE_ID\"]}" \
  -o json
```

### Pagination

List commands auto-paginate (all results). For single-page control:

```bash
anyclient objects list abc123 --limit 10 --offset 20 -o json
```

## IDs

All IDs are hex strings (e.g. `abc123def456`). Extract from JSON output with `jq -r '.id'` or `jq -r '.[].id'`.
