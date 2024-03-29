#! /bin/bash
MG=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d
TG=4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY
# valgrind --leak-check=full --main-stacksize=10000000 --num-callers=500 

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
--accounts-index-memory-limit-mb 250 \
--no-poh-speed-test \
--only-known-rpc \
--accountsdb-plugin-config /root/solana-proofs/config.json \
--known-validator 5D1fNXzvv5NjV1ysLjirC4WY92RNsVH18vjmcszZd8on \
--known-validator eoKpUABi59aT4rR9HGS3LcMecfut9x7zJyodWWP43YQ \
--known-validator nqkFSApiR4mTENxvmQwsNy9hXF2MGb6NqCA4Epmp7AH \
--expected-genesis-hash $TG 



# --no-genesis-fetch \
# --entrypoint entrypoint.testnet.solana.com:8001 \
# --entrypoint entrypoint2.testnet.solana.com:8001 \
# --entrypoint entrypoint3.testnet.solana.com:8001 \
