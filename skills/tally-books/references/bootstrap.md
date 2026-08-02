# Bootstrapping a Tally company (first run)

Do this before any statement import when the company is new, unfamiliar, or nearly empty.
The goal is to land on: the **right company**, a **usable chart of accounts**, and **posting
dates Tally will accept**.

## 1. Questions to ask up front

Ask these before creating or posting anything. Batch them.

| # | Ask | Why it matters |
|---|-----|----------------|
| 1 | **Which company** are we booking into? (exact name) | Multiple companies can be open; the gateway acts on the *active* one unless pinned. Pin with `TALLY_COMPANY`. |
| 2 | Is this the **correct/only company**, or a test/demo one? | A near-empty chart (2–5 ledgers) usually means fresh setup or wrong company loaded. |
| 3 | Is Tally **licensed or Educational**? | Educational mode restricts voucher dates (see §4). Real clients are licensed; then any date works. |
| 4 | What is the **financial year / books-beginning** date? | Vouchers before books-beginning are rejected. Confirm it covers the statement period. |
| 5 | Which **bank ledger** is this statement's account (or create one)? | Every Payment/Receipt needs the bank leg. |
| 6 | May I **create ledgers** as needed, and under which groups? | Hard rule: never invent ledgers silently. |
| 7 | Do you want **opening balances** set (bank, debtors, creditors)? | Without them, ledgers show only movement, not real balances — reconciliation won't match the statement. |

## 2. Confirm the active company

```bash
tally-ping                 # prints the active company
TALLY_COMPANY="Exact Name" tally-ledgers   # pin + inspect chart
```

If several companies are open, **always** set `TALLY_COMPANY` on every command so nothing
lands in the wrong book.

## 3. Creating a new company

**Company creation is NOT possible over the XML gateway** — Tally rejects it with
`The Base Currency Symbol is required!` and the symbol can't be supplied via XML. Create it
in the UI:

1. Gateway of Tally → **F3 → Company → Create Company** (or **Alt+F3**).
2. Name (exact — you'll pin it with `TALLY_COMPANY`), **Financial year beginning** and
   **Books beginning from** = start of the period (e.g. `1-Apr-2026`), base currency `₹`.
3. **Ctrl+A** to accept. TallyPrime loads it automatically.
4. Verify with `tally-ping` (active company should now be the new one).

## 4. Educational-mode date restriction (critical)

Unlicensed / **Educational** TallyPrime accepts voucher dates **only on day 1, 2, or 31** of a
month. It is *not* a true "last day" — 30-day month-ends (30 Apr, 30 Jun) are **rejected**.

Symptoms: voucher import returns `<ERRORS>1</ERRORS>` with an **empty `<LINEERROR/>`** and
nothing is created. The same voucher on day 1/2/31 succeeds.

Handling:

- **Ask before shifting dates.** Do not silently move a transaction's date.
- If the user accepts shifting: post each voucher on an allowed day **in its own month**
  (prefer `31` where the month has it, else `01`/`02`), and **preserve the real date in the
  narration**, e.g. `... | actual 20260618`.
- Months without a 31st (Apr, Jun, Sep, Nov, Feb) have no valid month-end in Educational mode —
  use `01`/`02`.
- Best fix is a license: once licensed, post real dates and skip all of this.

Quick check of which dates a given Tally will accept:

```bash
for D in 20260401 20260402 20260430 20260531 20260630; do
  tally-create-voucher --type Payment --date "$D" --bank "<bank>" --account "<any>" \
    --amount 1 --narration "probe-$D" >/dev/null 2>&1 && echo "$D OK" || echo "$D FAIL"
done
```

(These probes create ₹1 vouchers — see §7 on removing them.)

## 5. Building the chart of accounts

Propose the full ledger list (name + Tally group) and get sign-off **before** creating,
flagging anything ambiguous (party vs expense, employee salary vs contractor, tax
classification). Then:

```bash
TALLY_COMPANY="..." cargo run --example create_ledger -- \
  --name "Kotak Bank 2547392211" --parent "Bank Accounts"
```

Common groups: `Bank Accounts`, `Sundry Debtors`, `Sundry Creditors`, `Indirect Expenses`,
`Duties & Taxes`, `Current Liabilities`. Prefer existing ledgers; create only with permission.

## 6. Opening balances

A fresh company has no opening balances, so a bank ledger will show only period movement, not
the real statement balance. If reconciliation to the actual balance is wanted, set the opening
balance as of books-beginning:

```bash
cargo run --example create_ledger -- --name "Kotak Bank 2547392211" \
  --parent "Bank Accounts" --opening-balance 5460416.32
```

(`create_ledger` with an existing name alters it.) Confirm the figure and date with the user.

## 7. Verifying and cleanup

**Inspect entries (UI):** Gateway → **Display → Account Books → Ledger** → pick the bank
ledger → set period with **Alt+F2**. **F12** → show narrations to see the `actual …` dates.

**Check a balance (XML):** object export of the ledger's `CLOSINGBALANCE`.

**Deleting vouchers:** there is **no working XML delete** — gateway-created vouchers reject
delete by `MASTERID`, `VCHKEY`, and `REMOTEID` alike. Remove test/wrong entries in the UI:
Day Book → drill into the voucher → **Alt+D**.
