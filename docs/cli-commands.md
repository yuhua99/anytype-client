# Supported CLI commands

Baseline command inventory for refactor safety. Keep this file current when command surface changes.

Global options:

```text
--base-url <URL>
--api-key <KEY>
--config <PATH>
-o, --output <table|json|yaml>
```

Pagination options used by list/search-like commands:

```text
--limit <N>
--offset <N>
```

## Auth

```bash
anyclient auth desktop [--app-name anyclient] [--force]
anyclient auth headless [--api-key KEY] [--force]
```

## Spaces

```bash
anyclient spaces list [--limit N] [--offset N]
anyclient spaces get SPACE
anyclient spaces create --name NAME [--description DESC]
anyclient spaces update SPACE [--name NAME] [--description DESC]
```

## Objects

```bash
anyclient objects list SPACE [--limit N] [--offset N]
anyclient objects get SPACE OBJECT_ID [--format md]
anyclient objects create SPACE --name NAME [--type TYPE] [--body BODY] [--template TEMPLATE] [ICON] [PROPERTIES]

Example:
```bash
anyclient objects create space-id --name "My Task" --type task \
  --property '{"key":"status","text":"done"}' \
  --property '{"key":"tags","multi_select":["tag1","tag2"]}' -o json
```
anyclient objects update SPACE OBJECT_ID [--name NAME] [--type TYPE] [--markdown BODY] [ICON] [PROPERTIES] [--tag-property PROP --tag-add TAG --tag-remove TAG]

Example (properties + tags):
```bash
anyclient objects update space-id obj-id --name Updated \
  --tag-property status --tag-add done \
  --property '{"key":"title","text":"foo"}' -o json
```
anyclient objects delete SPACE OBJECT_ID
anyclient objects export SPACE OBJECT_ID [--format md]
anyclient objects update-many SPACE [--ids-file PATH] [--ids ID1,ID2] [--query QUERY] [--types TYPE1,TYPE2] [--tag-property PROP] [--tag-add TAG] [--tag-remove TAG] [--dry-run]
anyclient objects find SPACE [--type TYPE] [--tag TAG --tag-property PROP] [--property KEY=VALUE] [--name TEXT] [--missing-property PROP] [--ids-only] [--names-only]
anyclient objects count SPACE [--group-by type|property:KEY]
```

## Search

```bash
anyclient search [--query QUERY] [--types TYPE1,TYPE2] [--sort created_date|last_modified_date|last_opened_date|name] [--direction asc|desc] [--filters JSON] [--space SPACE] [--limit N] [--offset N]
```

Typed filter example:

```bash
anyclient search --space abc123 \
  --filters '{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}' \
  -o json
```

Legacy raw filter example (preserved for compatibility):

```bash
anyclient search --filters '{"type":"and","filters":[{"key":"type","condition":"equal","value":"task"}]}' -o json
```

Typed filter expressions (with `operator`/`conditions`) are also accepted and validated strictly.

## Types

```bash
anyclient types list SPACE [--limit N] [--offset N]
anyclient types get SPACE TYPE_ID
anyclient types create SPACE --name NAME --plural-name NAME --layout basic|profile|action|note [--key KEY] [ICON] [PROPERTY LINKS]
anyclient types update SPACE TYPE_ID [--name NAME] [--plural-name NAME] [--layout basic|profile|action|note] [--key KEY] [ICON] [PROPERTY LINKS]
anyclient types delete SPACE TYPE_ID
anyclient types templates SPACE TYPE_ID [--limit N] [--offset N]
anyclient types template-get SPACE TYPE_ID TEMPLATE_ID
```

## Properties

```bash
anyclient properties list SPACE [--limit N] [--offset N]
anyclient properties get SPACE PROPERTY_ID
anyclient properties create SPACE --name NAME --format text|number|select|multi_select|date|files|checkbox|url|email|phone|objects [--key KEY] [--tag TAG] [--tags-json JSON]
anyclient properties update SPACE PROPERTY_ID --name NAME [--key KEY]
anyclient properties delete SPACE PROPERTY_ID
```

## Tags

```bash
anyclient tags list SPACE PROPERTY_ID [--limit N] [--offset N]
anyclient tags get SPACE PROPERTY_ID TAG_ID
anyclient tags create SPACE PROPERTY_ID --name NAME --color grey|yellow|orange|red|pink|purple|blue|ice|teal|lime [--key KEY]

Example:
```bash
anyclient tags create space-id prop-id --name done --color lime -o json
```
anyclient tags update SPACE PROPERTY_ID TAG_ID [--name NAME] [--color COLOR] [--key KEY]
anyclient tags delete SPACE PROPERTY_ID TAG_ID
```

## Files

```bash
anyclient files upload SPACE PATH

Example:
```bash
anyclient files upload space-id photo.png -o json
```
anyclient files download SPACE FILE_ID --output PATH [--width N] [--force]
anyclient files delete SPACE FILE_ID [--skip-bin]
```

## Collections

```bash
anyclient collections views SPACE COLLECTION_ID [--limit N] [--offset N]
anyclient collections objects SPACE COLLECTION_ID VIEW_ID [--limit N] [--offset N]
anyclient collections add SPACE COLLECTION_ID OBJECT_ID...
anyclient collections remove SPACE COLLECTION_ID OBJECT_ID
```

## Members

```bash
anyclient members list SPACE [--limit N] [--offset N]
anyclient members get SPACE MEMBER_ID
```

## Shared argument groups

Icon options:

```text
--icon-emoji EMOJI
--icon-file FILE_ID
--icon-name NAME
--icon-color grey|yellow|orange|red|pink|purple|blue|ice|teal|lime
--clear-icon
```

Property value options:

```text
--property JSON
--properties-json JSON
```

Property link options:

```text
--property KEY_OR_ID
--properties KEY1,KEY2
--properties-json JSON
```
