# Anchor Vault Program

A simple Solana vault built with Anchor that lets a user initialize a vault, deposit SOL, withdraw SOL, and close the vault.

## Features

* Initialize vault
* Deposit SOL
* Withdraw SOL
* Close vault and reclaim funds
* Test cases covering all features

## Stack

* Solana
* Anchor
* TypeScript tests

## Run

```bash
anchor build
anchor test
```

## Run on local validator

```bash
anchor build
surfpool start (in separate terminal)
anchor test --skip-local-validator
```
---

## Screenshot of Test cases passed
<img width="2256" height="1412" alt="image" src="https://github.com/user-attachments/assets/003c5217-805d-4257-8a0c-5ea9a47e2b68" />
