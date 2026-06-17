# Tool reference

67 tools organised by category. Write tools are hidden when `GRAMPS_READONLY=true`.

## Search

Full-text search — returns a list of matching objects.

| Tool | Description |
| ---- | ----------- |
| `find_person` | Search people |
| `find_family` | Search families |
| `find_event` | Search events |
| `find_place` | Search places |
| `find_note` | Search notes |
| `find_tag` | Search tags |
| `find_citation` | Search citations |
| `find_media` | Search media objects |
| `find_repository` | Search repositories |
| `find_source` | Search sources |
| `find_anything` | Full-text search across all object types |

## Get

Fetch a single object or computed result by handle.

| Tool | Description |
| ---- | ----------- |
| `get_person` | Full person record |
| `get_family` | Full family record |
| `get_event` | Full event record |
| `get_place` | Full place record |
| `get_citation` | Full citation record |
| `get_note` | Full note record |
| `get_media` | Full media object record |
| `get_repository` | Full repository record |
| `get_tag` | Full tag record |
| `get_source` | Full source record |
| `get_relations` | Most direct relationship path between two people |
| `get_person_timeline` | Chronological event timeline for a person |
| `get_family_timeline` | Chronological event timeline for a family |
| `get_event_span` | Time span between two events |
| `get_tree_info` | Tree-level statistics and metadata |
| `list_transactions` | Recent transactions (use with `undo_transaction`) |

## Create *(write)*

| Tool | Description |
| ---- | ----------- |
| `create_person` | New person with optional name and gender |
| `create_family` | New family linking father and/or mother |
| `create_event` | New event (birth, death, marriage, …) |
| `create_place` | New place record |
| `create_source` | New source record |
| `create_note` | New text note |
| `create_tag` | New tag with name, colour, priority |
| `create_citation` | New citation linking a source |
| `create_repository` | New repository record |
| `create_media` | New media record from a server-side path or URL |

## Update *(write)*

Pass the full object from the corresponding `get_*` with modifications.

| Tool |
| ---- |
| `update_person` · `update_family` · `update_event` · `update_place` · `update_source` |
| `update_note` · `update_citation` · `update_repository` · `update_tag` · `update_media` |

## Delete *(write)*

| Tool |
| ---- |
| `delete_person` · `delete_family` · `delete_event` · `delete_place` · `delete_source` |
| `delete_note` · `delete_citation` · `delete_repository` · `delete_tag` · `delete_media` |

## Merge *(write)*

`survivor_handle` is kept; `duplicate_handle` is deleted.

| Tool | Extra params |
| ---- | ------------ |
| `merge_person` | `family_merger` (default `true`) |
| `merge_family` | `phoenix_father_handle`, `phoenix_mother_handle` |
| `merge_citation` · `merge_event` · `merge_media` · `merge_note` · `merge_place` · `merge_repository` · `merge_source` | — |

## Transactions *(write)*

| Tool | Description |
| ---- | ----------- |
| `undo_transaction` | Undo a transaction by ID obtained from `list_transactions` |
