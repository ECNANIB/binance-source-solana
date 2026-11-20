<p align="center">
  <img src="assets/banner.png" alt="ECNANIB: UFAS ERA SDNUF" width="100%">
</p>

<h1 align="center">
  <img src="assets/icon.png" alt="" width="36" valign="middle">&nbsp;binance-source-solana
</h1>

<p align="center">
  <img src="https://img.shields.io/badge/byte--identical-11%20programs-2ea043" alt="byte-identical: 11 programs">
  <img src="https://img.shields.io/badge/in%20progress-3-d29922" alt="in progress: 3">
  <img src="https://img.shields.io/badge/Binance%20verified%20builds-0-cf222e" alt="Binance verified builds: 0">
  <img src="https://img.shields.io/badge/Solana-mainnet-9945FF?logo=solana&logoColor=white" alt="Solana mainnet">
  <img src="https://img.shields.io/badge/Rust-Anchor-dea584?logo=rust&logoColor=white" alt="Rust / Anchor">
  <img src="https://img.shields.io/badge/build-reproducible-0969da" alt="build: reproducible">
</p>

<p align="center">
  <b>Binance runs its Solana programs closed source: no published code, no verified builds, no on-chain IDL.</b><br>
  Here is source for eleven of them. Every one compiles <b>byte-for-byte</b> to the binary deployed on mainnet.<br>
  Don't take our word for it: <a href="#check-it-yourself">build them and compare</a>.
</p>

## Programs

| directory | program id | current build deployed (slot) | sha256 of deployed ELF |
|---|---|---|---|
| [`router`](#router) | `B3111yJCeHBcA1bizdJjUFPALfhAfSRnAbJzGUtnt56A` | 2026-04-23 (415118680) | `cb1df04a6e0652ac…` |
| [`w3w-vault`](#w3w-vault) | `BN111AnCthcdPVNJ6jkir9TDaS7xqXT8EhetAmYpNqFt` | 2026-01-14 (393409008) | `dc345765bdf1df17…` |
| [`cloud-wallet`](#cloud-wallet) | `BN111TK638zTi7uSXCKkzXYPSRt3rKNSDBK6VsjS772b` | 2025-08-27 (362808756) | `e543516ebf83f198…` |
| [`memeshot-vault-sol`](#memeshot-vault-sol) | `BN111JnbLtbmQqqiCh7h2pDKhAhMx4wi77Mj7jJFbyp8` | 2025-04-18 (334282679) | `2d3202a695f1fd91…` |
| [`deposit-agent`](#deposit-agent) | `BN111erRrQy7bpcZBEH7i9anCsrzkSHRFwmJhXiyWitm` | 2025-04-29 (336638651) | `b767b8ae240915e9…` |
| [`router-nbEkL7`](#router-nbekl7) | `nbEkL7pwLvMdDToFY8bsoTUyFJ5Y9NtJvW2wMHMWzwH` | 2026-10-26 (461599332) | `6918e7e2fc4edcad…` |
| [`cloud-wallet-9s8e9k`](#cloud-wallet-9s8e9k) | `9s8e9kamL24rB79jhuRsuBRPnwd6H2LHRXKkH18USW85` | 2025-04-21 (334844880) | `4627ff1429baf8a7…` |
| [`claim`](#claim) | `NBAgSorjYjpnWXfJ7SwdhNUrzoXGJ9ysGqoYihrsNgp` | 2025-09-10 (365849928) | `59bd77bfd483cc00…` |
| [`vault`](#vault) | `nbEursuNQy9R8XeQH7yQYuKvVupqorCTmXJsvWskQGy` | 2025-09-10 (365872329) | `943fe61c44455c9c…` |
| [`referral-swap`](#referral-swap) | `9VdG8gJNUo5r7MtGb4NHVP8tpnvoGZWJcByTGDXXtbY1` | 2026-03-25 (408735911) | `b3a24037eb26a000…` |
| [`deposit-agent-5UK8Rc`](#deposit-agent-5uk8rc) | `5UK8RcSv4DiDpHSEsBu8y9qnqzizdyDsNbNpSRdc4Ko` | 2026-11-18 (470780163) | `e8a75939de40feaa…` |

Full hashes are in each `programs/<name>/build.conf`. Directory names are the programs' own crate names, taken
from the binaries; where two deployed programs are the same crate, the second carries the start of its id.

## Progress

Share of each program's functions that compile identical to mainnet. A program is published once all of its
functions match and the whole binary hashes the same.

| program | progress | functions | |
|---|---|---|---|
| [`router`](#router) | `████████████████████` 100% | 334 / 334 | ✅ byte-identical |
| [`w3w-vault`](#w3w-vault) | `████████████████████` 100% | 478 / 478 | ✅ byte-identical |
| [`cloud-wallet`](#cloud-wallet) | `████████████████████` 100% | 499 / 499 | ✅ byte-identical |
| [`memeshot-vault-sol`](#memeshot-vault-sol) | `████████████████████` 100% | 449 / 449 | ✅ byte-identical |
| [`deposit-agent`](#deposit-agent) | `████████████████████` 100% | 360 / 360 | ✅ byte-identical |
| [`router-nbEkL7`](#router-nbekl7) | `████████████████████` 100% | 424 / 424 | ✅ byte-identical |
| [`cloud-wallet-9s8e9k`](#cloud-wallet-9s8e9k) | `████████████████████` 100% | 488 / 488 | ✅ byte-identical |
| [`claim`](#claim) | `████████████████████` 100% | 375 / 375 | ✅ byte-identical |
| [`vault`](#vault) | `████████████████████` 100% | 359 / 359 | ✅ byte-identical |
| [`referral-swap`](#referral-swap) | `████████████████████` 100% | 367 / 367 | ✅ byte-identical |
| [`deposit-agent-5UK8Rc`](#deposit-agent-5uk8rc) | `████████████████████` 100% | 345 / 345 | ✅ byte-identical |
| `cloud-wallet-378Ebb` | `███████████████████▉` 99.6% | 505 / 507 | in progress |
| `memeshot-vault-sol-HSJKDe` | `███████████████████░` 95.4% | 453 / 475 | in progress |
| `w3w-vault-56fYxT` | `██████████████████▊░` 94.2% | 458 / 486 | in progress |

In progress: `cloud-wallet-378Ebb` (`378EbBbP1XVrR2N4ED7vxVpMbXXiBv9NjYhNjNi8GxeR`), `w3w-vault-56fYxT`
(`56fYxTwKZRjteZKZQWmVTVzhmtNiHRBdxCFcRAJ9QP4e`) and `memeshot-vault-sol-HSJKDe`
(`HSJKDego6Ti1gqg3e8DMYD2zgiBkMjHLRZvSEXLNBdHu`), all redeployed by Binance on 2026-11-18. Updated 2026-11-20.

**Who runs them.** On Solscan, `B3111y…` is labelled *Binance Wallet Program*, `BN111TK…` *Binance Alpha 2.0
Program 2* and `BN111Jnb…` *Binance Alpha 2.0 Program 1*. The first five programs are held by production upgrade
authorities. The other six are held by `DqSVfVxQN3sJNmecje6e7b7kesDb2VBuBMK6oXdWF2Mo`, the wallet that deployed
every one of the first five before handing it to its production authority. All eleven are upgradeable.

## What each program does

Click a program to expand it.

Most of these programs share one design: tokens sit in a single program-owned account (`token_holder`), and
only keys on an allow-list stored in the program's config can move them. Many actions also need a secp256k1
(Ethereum-style) signature from a key in that config, over a message that names the chain, the program, the
action, a single-use nonce and an expiry. Every program can be paused by its admin.

<a name="router"></a>
<details>
<summary><b><code>router</code></b> · <i>Binance Wallet Program</i> · the swap entry point, with fee capture</summary>

*Binance Wallet Program.* The user-facing swap entry point. The user signs; the program calls one of a fixed set
of approved aggregator programs to do the swap. The `*_fee_log` variants take a fee in basis points, from the input
or from what the swap delivered, and send it to a fee account whose owner must be on the program's approved list,
logging an event. Also: batch create and close token accounts, and wrapped-SOL sync. 8 instructions.

</details>

<a name="router-nbekl7"></a>
<details>
<summary><b><code>router-nbEkL7</code></b> · next <code>router</code> build · positive-slippage capture, dynamic input, delta windows</summary>

The same `router` crate, a later build, deployed under the build wallet. Everything in `router`, plus:
- **Positive-slippage swaps** (`swap_with_positive_slippage` v1–v4, token and SOL). After the swap, whatever the
  user received above a caller-supplied expected amount (after commission) is moved out of the user's token
  account to a receiver that must be on the program's approved list.
- **Dynamic-input swaps.** Rewrites the inner swap's input amount at run time, keeping back a reserved amount, and
  fails if the aggregator spends more than the budget. It decodes and checks the outer instruction that called it.
- **Delta windows** (`begin_*_delta_window` / `settle_*_delta_window`, for a token account or a SOL wallet).
  Reading the Instructions sysvar, `begin` snapshots a watched balance and requires a matching `settle` later in
  the same transaction; `settle` fails unless the balance went up by at least a stated minimum, then sends the
  gain to a recipient.

21 instructions.

</details>

<a name="w3w-vault"></a>
<details>
<summary><b><code>w3w-vault</code></b> · Web3 Wallet vault · operator-run swaps out of a pooled account</summary>

"W3W" is Binance's short name for its Web3 Wallet. Holds pooled tokens. Allow-listed operator keys swap the pool's
tokens through an approved aggregator and deliver the output to a user's token account (`swap_to_token`,
`swap_to_sol`, and `_v2` versions), with a single-use nonce, an expiry and a per-token maximum trade size. Operators
can also move funds to approved rebalance addresses (`rebalance_out`, `rebalance_out_sol`). Admin: trade-token
list, secp256k1 keys, pause, emergency withdraw. 20 instructions.

</details>

<a name="cloud-wallet"></a>
<details>
<summary><b><code>cloud-wallet</code></b> · <i>Binance Alpha 2.0 Program 2</i> · pooled custody, signature-gated withdrawals</summary>

*Binance Alpha 2.0 Program 2.* Pooled custody. Deposits go into one program-owned account and emit an event
tagged with a Binance account id and deposit id; `user_deposit` additionally needs a Binance-signed authorization.
Withdrawals (`withdraw`, `withdraw_v2`) and swaps are sent by allow-listed operator keys and must carry a
secp256k1 signature from a Binance key over the chain, program, nonce, mint, amount, recipient and expiry. Nonces
are tracked in on-chain bitmaps, withdrawable tokens are allow-listed, and a per-user withdraw gate can block a
withdrawal. `quick_withdraw` lets a separate set of operator keys pull funds without a signature. Swaps go through
approved aggregators. Admin: pause, emergency withdraw, ownership transfer, allow-lists, `upgrade_to_v4`.
29 instructions.

</details>

<a name="cloud-wallet-9s8e9k"></a>
<details>
<summary><b><code>cloud-wallet-9s8e9k</code></b> · earlier <code>cloud-wallet</code> build</summary>

An earlier build of `cloud-wallet` under the build wallet: it has `upgrade_to_v3` but not `upgrade_to_v4` or
`user_deposit`. 28 instructions.

</details>

<a name="memeshot-vault-sol"></a>
<details>
<summary><b><code>memeshot-vault-sol</code></b> · <i>Binance Alpha 2.0 Program 1</i> · buys tokens and deposits them into <code>cloud-wallet</code></summary>

*Binance Alpha 2.0 Program 1.* The trading side of Alpha, working with `cloud-wallet`. `buy_to_alpha`: an
allow-listed operator swaps the vault's tokens through an approved aggregator, then calls `cloud-wallet`'s
`deposit` with the tokens bought. `sell_to_self_from_alpha`: pulls tokens out of `cloud-wallet` with its
`quick_withdraw` and swaps them back. Per-token maximum trade size, rebalance out, admin. 16 instructions.

</details>

<a name="deposit-agent"></a>
<details>
<summary><b><code>deposit-agent</code></b> · per-account deposit addresses, swept with signed authorizations</summary>

Per-account deposit addresses. Every account id maps to its own program-derived address (`get_address` returns
it). Allow-listed operator keys sweep tokens from those addresses to one configured destination (`collect_to`),
each sweep authorized by a secp256k1 signature from a configured key. 9 instructions.

</details>

<a name="deposit-agent-5uk8rc"></a>
<details>
<summary><b><code>deposit-agent-5UK8Rc</code></b> · the 2026-11-18 <code>deposit-agent</code> build</summary>

A newer build of `deposit-agent` under the build wallet, redeployed on 2026-11-18 alongside three other programs
under that wallet. Same nine instructions, now built with platform-tools v1.52, with shared helpers split into a
`utils` library.

</details>

<a name="claim"></a>
<details>
<summary><b><code>claim</code></b> · signature-gated payouts</summary>

Signature-gated payouts. A user claims an amount when they hold a signed authorization over the program, the claim
id, the amount, a fee, their address and a deadline. Claim ids are single-use (on-chain bitmaps).
`claim_token_batch` does the same for tokens. 11 instructions.

</details>

<a name="vault"></a>
<details>
<summary><b><code>vault</code></b> · payment splitter</summary>

A payment splitter. An allow-listed key pays out of the program-owned account, splitting each payment (one-time
id) between a configured platform address and a configured split address, in SOL or tokens
(`split_payment`, `split_payment_token_batch`). 9 instructions.

</details>

<a name="referral-swap"></a>
<details>
<summary><b><code>referral-swap</code></b> · operator swaps through one configured router</summary>

Allow-listed operator keys swap the program's pooled tokens through one configured router program (`swap`) and
send tokens and SOL to a configured platform address (`rebalance_to`). 8 instructions.

</details>

## Check it yourself

Linux x86_64 with `curl`, `tar`, `bzip2`, `python3`:

```sh
./build.sh all                                   # or: ./build.sh router
python3 onchain_sha256.py B3111yJCeHBcA1bizdJjUFPALfhAfSRnAbJzGUtnt56A
```

`build.sh` downloads Anza's published
[platform-tools](https://github.com/anza-xyz/platform-tools/releases) for each program's release, builds it,
strips it and compares against the deployed hash. `onchain_sha256.py` reads the program straight from mainnet RPC
and prints the hash of the deployed ELF, so you can confirm the expected value too.

Most of these programs were built on a Mac. For those, `build.sh` links the standard library from the macOS
platform-tools archive of the same release and tells cargo the original host (`HOST` in `build.conf`). The host
feeds the hash cargo gives every crate, so the bytes depend on it.

## Notes

- **Names.** Program, instruction, account, error and file names come from the binaries. Names the binary does not
  record (arguments, struct fields, locals, helpers) cannot be recovered and may not match Binance's; a few are
  plainly placeholders.
- **Upgrades.** A redeploy changes the deployed hash; `build.sh` then reports `DIFFER` until the source catches up.
