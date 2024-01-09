#! /bin/bash

exec target/release/solana-validator \
--identity keypair.json \
--entrypoint entrypoint.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint2.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint3.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint4.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint5.mainnet-beta.solana.com:8001 \
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
--no-genesis-fetch \
--no-snapshot-fetch \
--snapshots /mnt/ssd1/snapshots \
--limit-ledger-size 400000000 \
--rpc-send-default-max-retries 3 \
--rpc-send-service-max-retries 3 \
--rpc-send-retry-ms 2000 \
--full-rpc-api \
--accounts-index-memory-limit-mb 350 \
--no-poh-speed-test \
--only-known-rpc \
--known-validator sce1TVNf6tBniHXqhkj7NY9wxaBsoQrpXKzfYXBG9v2 \
--known-validator sce3PzrJU8j5Qa7WtRejXq7hFywUBh8jutVx929Hagw \
--known-validator sce2qRjzVVeH8um2zrTfF7RhLCz2jYiiu8m3R6cskZc \
--expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d
