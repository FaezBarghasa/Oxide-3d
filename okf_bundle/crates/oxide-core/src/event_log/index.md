# event_log

## Classs

- [EventLog](EventLog.md) — Event-Sourced Document Operation Log.
- [OperationPayload](OperationPayload.md) — Generic atomic operational delta applied to the document.
- [OperationRecord](OperationRecord.md) — Immutable historical entry in the document's event stream.
- [OperationStatus](OperationStatus.md) — Status of an operation in the event log.

## Functions

- [active_entries](active_entries.md) — Slice of currently active operations (up to cursor).
- [active_entries](active_entries_1.md) — Slice of currently active operations (up to cursor).
- [append](append.md) — Appends and applies a new operation to the log, truncating any rolled-back redo history.
- [append](append_1.md) — Appends and applies a new operation to the log, truncating any rolled-back redo history.
- [cursor](cursor.md) — Current evaluation cursor position.
- [cursor](cursor_1.md) — Current evaluation cursor position.
- [entries](entries.md) — Slice of all operations.
- [entries](entries_1.md) — Slice of all operations.
- [is_empty](is_empty.md) — Whether the log is empty.
- [is_empty](is_empty_1.md) — Whether the log is empty.
- [len](len.md) — Number of total operations recorded.
- [len](len_1.md) — Number of total operations recorded.
- [new](new.md) — Create a new applied operation record with generated UUIDv7.
- [new](new_1.md) — Create a new applied operation record with generated UUIDv7.
- [new](new_2.md) — Creates an empty event log.
- [new](new_3.md) — Creates an empty event log.
- [redo](redo.md) — Redo one operation by advancing the evaluation cursor.
- [redo](redo_1.md) — Redo one operation by advancing the evaluation cursor.
- [set_cursor](set_cursor.md) — Move the rollback bar to a specific historical operation index $k \in [0, N]$.
- [set_cursor](set_cursor_1.md) — Move the rollback bar to a specific historical operation index $k \in [0, N]$.
- [test_event_log_append_undo_redo](test_event_log_append_undo_redo.md) — [test]
- [undo](undo.md) — Undo one operation by stepping the evaluation cursor back.
- [undo](undo_1.md) — Undo one operation by stepping the evaluation cursor back.
