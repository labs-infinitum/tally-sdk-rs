---
name: tally-books
description: >
  Connects to a local TallyPrime instance (XML/HTTP on localhost:9000) to inspect
  ledgers and post accounting vouchers. Use when the user provides a bank statement,
  asks to record bank transactions into Tally, reconcile books, create payment/receipt
  vouchers, list Tally ledgers, or bookkeep for an Indian Pvt Ltd (especially software
  services). Always ask clarifying questions before posting anything.
license: Apache-2.0
compatibility: Requires Rust/cargo, a reachable TallyPrime on localhost:9000, and this repo's local tally-sdk-rust path dependency.
metadata:
  author: labs-infinitum
  version: "0.1.0"
  openclaw:
    requires:
      bins: ["cargo", "rustc"]
---

# Tally Books

Talk to Tally through this repo's Rust SDK. Do **not** invent ledger names or post vouchers until the user confirms ambiguous mappings.

## Ask first (every session)

Before creating or posting anything, confirm the ground you're standing on:

1. **Which company** are we booking into? (exact name — pin it with `TALLY_COMPANY` if several are open)
2. Is Tally **licensed or Educational**? Educational mode only accepts voucher dates on **day 1, 2, or 31** (see [Known Tally constraints](#known-tally-constraints)).
3. Does the company have a **usable chart of accounts**, or are we bootstrapping? A near-empty ledger list (2–5 ledgers) means fresh setup or the wrong company.
4. Which **bank ledger** is this statement's account, and do you want **opening balances** set?

If the company is new/empty, or the wrong one is loaded, follow **[references/bootstrap.md](references/bootstrap.md)** before anything else. New companies **cannot** be created over XML — that's a UI step.

## Setup (once per machine)

From the skill directory (or repo root):

```bash
bash scripts/install.sh
```

This:

1. Resolves the local `tally-sdk-rs` checkout
2. Builds the skill CLI with a Cargo path dependency on that checkout
3. Symlinks this skill into `~/.claude/skills/tally-books` for Claude Code / Claude Desktop filesystem skills

For Claude Desktop ZIP upload: zip the `tally-books` folder (so `SKILL.md` is at the zip root), then add it under Customize → Skills. Prefer the symlink install when the app can read `~/.claude/skills`.

Env overrides (optional):

- `TALLY_HOST` (default `localhost`)
- `TALLY_PORT` (default `9000`)
- `TALLY_COMPANY` (recommended when multiple companies are open)
- `TALLY_SDK_ROOT` (set by install; path to this repo)

## CLI tools

Run via the built binaries (preferred) or `cargo run` from `scripts/`:

| Command | Purpose |
|---------|---------|
| `tally-ping` | Verify Tally is reachable; print active company |
| `tally-ledgers` | List ledger names (and parents) |
| `tally-create-voucher` | Create Payment / Receipt / Contra / Journal (2-line) |

Examples:

```bash
# From skills/tally-books/scripts after install/build:
./target/release/tally-ping
./target/release/tally-ledgers
./target/release/tally-create-voucher \
  --type Payment \
  --date 20260701 \
  --bank "HDFC Bank" \
  --account "Internet Charges" \
  --amount 1180 \
  --narration "July Jio fiber" \
  --dry-run
```

`--dry-run` prints the voucher plan without posting. Never post without user confirmation when the contra ledger is guessed.

## Bank statement workflow

When the user shares a bank statement (PDF/CSV/image/text):

1. **Connect** — run `tally-ping`. If it fails, stop and ask them to enable Tally XML on port 9000. Confirm the reported company is the right one; pin `TALLY_COMPANY` if multiple are open.
2. **Load chart** — run `tally-ledgers` (with `TALLY_COMPANY` set). If it's nearly empty, bootstrap first: [references/bootstrap.md](references/bootstrap.md). Keep the list for mapping.
3. **Parse statement** — extract date, narration, amount, direction (money in vs out), and bank reference per line.
4. **Identify bank ledger** — match statement account to a Tally bank ledger. If unclear, ask.
5. **Propose mappings** — for each line, propose voucher type + contra ledger:
   - Money **out** → `Payment` (Dr expense/party, Cr bank)
   - Money **in** → `Receipt` (Dr bank, Cr income/party)
   - Transfer between own accounts → `Contra`
6. **Ask before posting** — stop and ask when any of these are true:
   - No exact ledger match
   - Ambiguous narration (salary vs contractor vs director draw)
   - Possible GST-inclusive amount without tax breakup
   - New party / vendor not in ledgers
   - Duplicate-looking entry (same date+amount+narration)
7. **Confirm batch** — show a table of proposed vouchers; wait for approval (all / subset / edits).
8. **Post** — create approved vouchers one at a time with `tally-create-voucher` (no `--dry-run`). Report success/failure per line.
9. **Summarize** — totals posted, skipped, and open questions.

Detailed mapping heuristics for software-services Pvt Ltd: [references/bank-statement-import.md](references/bank-statement-import.md).

## Hard rules

- Prefer existing Tally ledgers over creating new ones. Create a ledger only if the user explicitly says to.
- Do not invent GST entries from a bank line unless the user provides tax breakup or confirms inclusive treatment.
- Dates must be `YYYYMMDD`. **Post the actual transaction date** — a licensed Tally accepts any date, so this is the normal case. Only Educational mode rejects most dates (see below); *then* ask before shifting, and if the user agrees, stay in-month and note the real date in the narration (`... | actual YYYYMMDD`). Never shift dates on a licensed instance.
- Never silently skip failed posts — surface the error and ask how to proceed.
- Keep narration close to the bank text; add a short clarified suffix only when helpful.
- When several companies are open, set `TALLY_COMPANY` on every command so entries can't land in the wrong book.

## Known Tally constraints

Learned the hard way; check these before blaming your XML:

- **Educational-mode dates** — unlicensed TallyPrime accepts voucher dates **only on day 1, 2, or 31** (not a true "last day"; 30 Apr / 30 Jun are rejected). Failure looks like `<ERRORS>1</ERRORS>` with an **empty `<LINEERROR/>`** and nothing created. A licensed Tally takes any date.
- **New companies can't be created over XML** — the gateway rejects it (`Base Currency Symbol is required!`). Create in the UI (F3 → Company → Create). See [references/bootstrap.md](references/bootstrap.md).
- **No working XML voucher delete** — gateway-created vouchers reject delete by `MASTERID`, `VCHKEY`, and `REMOTEID`. Remove wrong/test entries in the UI: Day Book → drill in → `Alt+D`.
- **Empty `<LINEERROR/>` = voucher/company-level rejection** (bad date, outside books period, wrong company), not a ledger-line problem. Re-run with `--debug` to see the raw request/response.
- **Fresh companies have no opening balances** — a bank ledger then shows only movement, not the real statement balance. Set opening balances if reconciliation is needed.

## Software services defaults (India Pvt Ltd)

Use only as **suggestions** when asking the user — never as silent defaults:

| Bank pattern | Likely ledger / type |
|--------------|----------------------|
| AWS, GCP, Azure, Vercel, GitHub | Cloud / hosting expense |
| Google Workspace, Slack, Notion | Software subscription expense |
| Client NEFT/RTGS + invoice ref | Receipt → Sundry Debtors party |
| Salary / payroll | Salary expense or payable |
| GST payment to govt | Duties & Taxes / GST ledgers |
| TDS remittance | TDS payable |
| Rent | Rent expense |
| Director reimbursement | Ask — expense vs loan/drawings |

## Progressive disclosure

- First-run / new or empty company → [references/bootstrap.md](references/bootstrap.md)
- Bank import details → [references/bank-statement-import.md](references/bank-statement-import.md)
- CLI flags / JSON shape → [references/cli.md](references/cli.md)
