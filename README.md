# Delivery Box with Auto-Payment

Smart delivery box that releases courier payment only after confirmed drop-off.

## Problem
Couriers in Quezon City lose money and time due to failed deliveries when recipients are unavailable.

## Solution
Buyers prepay using Stellar; funds are released instantly when courier confirms delivery via QR scan.

## Timeline
Day 1: Contract  
Day 2: QR + frontend  
Day 3: Demo  

## Stellar Features Used
- USDC transfers  
- Soroban smart contracts  

## Vision and Purpose
Eliminate failed deliveries and enable trustless last-mile logistics.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/delivery_box_with_auto_payment.wasm \
--network testnet

## Example Call
soroban contract invoke \
--id <CONTRACT_ID> \
--fn create_delivery \
--arg <buyer> \
--arg <courier> \
--arg 100

## License
MIT
