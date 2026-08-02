# Skill CLI reference

Built from `skills/tally-books/scripts` against the local SDK (`tally-sdk-rust` path dependency).

## Environment

| Variable | Default | Meaning |
|----------|---------|---------|
| `TALLY_HOST` | `localhost` | Tally HTTP host |
| `TALLY_PORT` | `9000` | Tally XML port |
| `TALLY_COMPANY` | _(active)_ | Force company name |
| `TALLY_SDK_ROOT` | set by `install.sh` | Repo root containing this SDK |

## `tally-ping`

```bash
tally-ping
```

Prints JSON: `{ "ok": true, "company": "..." }`.

## `tally-ledgers`

```bash
tally-ledgers
tally-ledgers --json
```

Default: one `name <TAB> parent` per line. `--json` prints an array of `{name,parent}`.

## `tally-create-voucher`

```bash
tally-create-voucher \
  --type Payment|Receipt|Contra|Journal \
  --date YYYYMMDD \
  --bank "Bank Ledger Name" \
  --account "Contra Ledger Name" \
  --amount 1234.56 \
  [--narration "..."] \
  [--voucher-number "..."] \
  [--dry-run] \
  [--debug]
```

Semantics:

| `--type` | Debit | Credit |
|----------|-------|--------|
| `Payment` | `--account` | `--bank` |
| `Receipt` | `--bank` | `--account` |
| `Contra` | `--account` (destination) | `--bank` (source) |
| `Journal` | `--account` | `--bank` |

Exit codes: `0` success (or dry-run), `1` validation/Tally error, `2` connection error.
