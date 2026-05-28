# anyclient

CLI for Anytype.

## Core rules
- Always use `-o json` for machine-readable output.
- Filters: typed only — `{"operator":"and","conditions":[...]}`.
- `--property` JSON: use `"key"`, never `"id"`.

## Common workflows

### Create task with properties + tags
```bash
anyclient objects create <space> --name "Fix login" --type task \
  --property '{"key":"status","select":"<tag-id>"}' \
  --tag-add <tag-id>
```

### Search with typed filter
```bash
anyclient search --space <space> --filters '{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}' -o json
```

### Bulk update (by query or IDs)
```bash
anyclient objects update-many <space> --query "status:doing" \
  --property '{"key":"status","select":"<done>"}'

anyclient objects update-many <space> --ids-file ids.txt \
  --tag-property "Tags" --tag-add <urgent>
```

### Upload file then attach
```bash
FILE=$(anyclient files upload <space> design.png -o json | jq -r '.id')
anyclient objects update <space> <obj> --property "{\"key\":\"attachments\",\"files\":[\"$FILE\"]}"
```

### Get IDs only (scripting)
```bash
anyclient objects find <space> --type task --tag "urgent" --ids-only
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
