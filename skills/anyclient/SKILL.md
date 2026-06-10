---
name: anyclient
description: Use for Anytype object management, note access, searches with typed filters, property/tag updates, file uploads, bulk operations, and scripting via anyclient. Use this when the user wants to access their notes.
---

# anyclient

CLI for Anytype.

## Core rules
- Always use `-o json` for machine-readable output.
- Filters: typed only — `{"operator":"and","conditions":[...]}`.
- `--property` JSON: use `"key"`, never `"id"`.

## Common workflows

### Create task with properties (tags via multi_select)
```bash
anyclient objects create <space> --name "Buy groceries" --type task \
  --property '{"key":"status","select":"<tag-id>"}' \
  --property '{"key":"tags","multi_select":["<tag-id>"]}'
```
`--tag-add`/`--tag-remove` only exist on `objects update` and `objects update-many`.

### Search with typed filter
```bash
anyclient search --space <space> --filters '{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}' -o json
```

### Bulk tag update (by query or IDs)
```bash
# --query is a full-text search string (not key:value syntax);
# update-many only supports tag add/remove, not --property.
anyclient objects update-many <space> --query "groceries" --types task \
  --tag-property "Tags" --tag-add <done> --dry-run

anyclient objects update-many <space> --ids-file ids.txt \
  --tag-property "Tags" --tag-add <urgent>
```

### Upload file then attach
```bash
FILE=$(anyclient files upload <space> design.png -o json | jq -r '.object_id')
anyclient objects update <space> <obj> --property "{\"key\":\"attachments\",\"files\":[\"$FILE\"]}"
```

### Get IDs only (scripting)
```bash
anyclient objects find <space> --type task --tag "urgent" --tag-property "Tags" --ids-only
```

### Count grouped by property
```bash
anyclient objects count <space> --group-by property:status -o json
```

### Add/remove tags on single object
```bash
anyclient objects update <space> <obj> \
  --tag-property "Tags" --tag-add <tag1> --tag-remove <tag2>
```

## Property value examples
```bash
--property '{"key":"status","select":"<id>"}'
--property '{"key":"tags","multi_select":["<id1>","<id2>"]}'
--property '{"key":"due_date","date":"2025-06-01"}'
--property '{"key":"assignee","objects":["<user-id>"]}'
```

Run `anyclient <command> -h` for full options and current enum values.
