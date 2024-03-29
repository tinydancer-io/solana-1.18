#! /bin/bash

exec target/release/solana-validator \
--identity keypair.json \
--entrypoint entrypoint.testnet.solana.com:8001 \
--entrypoint entrypoint2.testnet.solana.com:8001 \
--entrypoint entrypoint3.testnet.solana.com:8001 \
--rpc-port 8899 \
--dynamic-port-range 8002-8099 \
--gossip-port 8001 \
--no-voting \
--private-rpc \
--rpc-bind-address 0.0.0.0 \
--enable-cpi-and-log-storage \
--enable-rpc-transaction-history \
--wal-recovery-mode skip_any_corrupted_record \
--log /mnt/ssd1/validator.log \
--accounts /mnt/ssd1/accounts \
--ledger /mnt/ssd1/ledger \
--snapshots /mnt/ssd1/snapshots \
--limit-ledger-size 400000000 \
--rpc-send-default-max-retries 3 \
--rpc-send-service-max-retries 3 \
--rpc-send-retry-ms 2000 \
--full-rpc-api \
--accounts-index-memory-limit-mb 350 \
--no-poh-speed-test \
--no-snapshot-fetch \
--no-genesis-fetch \
--only-known-rpc \
--known-validator ELY6u8Reinx7k2s9wgQA6jHHTsog7eN6qAYeTh2bNYx \
--known-validator CV3F19YAhoW7DpfHQ5W9t2Zomb9h21NRi8k6hCA36Sk6 \
--known-validator DxQTxfgNhgzpRR8dZZku9XHPpWyy2Fb7oJSWoLTKwwKt \
--expected-genesis-hash 4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY
