# Bank statement → Tally

## Goal

Turn each bank line into a balanced accounting voucher against the company's bank ledger, with user confirmation for anything ambiguous.

## Parse checklist

For every statement line capture:

| Field | Notes |
|-------|--------|
| `date` | Normalize to `YYYYMMDD` |
| `amount` | Absolute value; keep direction separate |
| `direction` | `in` (credit) or `out` (debit) from the company's perspective |
| `narration` | Raw bank text |
| `ref` | UTR / cheque / card ref if present |

Ignore opening/closing balance rows and non-transaction headers.

## Voucher shapes

Amounts use Tally import convention:

- Debit: `ISDEEMEDPOSITIVE=Yes`, `AMOUNT=-X`
- Credit: `ISDEEMEDPOSITIVE=No`, `AMOUNT=+X`

### Payment (money out)

```
Dr <expense or party>   amount
Cr <bank ledger>        amount
```

### Receipt (money in)

```
Dr <bank ledger>        amount
Cr <income or party>    amount
```

### Contra (own-account transfer)

```
Dr <destination bank/cash>
Cr <source bank/cash>
```

## Clarifying questions (ask these)

Ask in one batch when possible:

1. Which **company** are we booking into (if ambiguous / several open)? Confirm licensed vs Educational — see bootstrap.md §4.
2. Which Tally ledger is this bank account? Set an **opening balance** as of books-beginning if reconciliation to the real balance is wanted.
3. For unmatched narrations: pick an existing ledger, create a new one (name + parent group), or skip?
4. Are salary lines gross salary, or net of deductions already?
5. For client receipts: which Sundry Debtor ledger / invoice bill-ref?
6. Should platform fees (Razorpay etc.) be split from settlements, or booked net?
7. Any lines to leave unbooked (owner personal, already recorded)?

## Duplicate guard

Before posting, scan recent vouchers (same bank, ±3 days, same amount) if the user can provide day-book context; otherwise warn on identical narration+date+amount within the statement itself.

## After posting

Report:

- Posted count + total out + total in
- Failures with Tally error text
- Lines still awaiting answers
