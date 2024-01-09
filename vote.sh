#! /bin/bash

content=$(curl http://localhost:$PORT -X POST -H "Content-Type: application/json" -d '
  {
    "id":1,
    "jsonrpc":"2.0",
    "method":"getLatestBlockhash",
    "params":[
      {
        "commitment":"confirmed"
      }
    ]
  }
  ')
  echo "done1"
  blockhash=$(jq -r '.result.context.slot' <<<"$content")


  echo $blockhash
#
#   #echo $body
  curl http://localhost:$PORT -X POST -H "Content-Type: application/json" -d @<( cat <<EOF
  {
    "id":1,
    "jsonrpc":"2.0",
    "method":"getBlockHeaders",
    "params":[
    $blockhash,

  { "commitment":"confirmed", "encoding": "jsonParsed",
        "maxSupportedTransactionVersion":0,
        "transactionDetails":"full",
        "rewards":false
      } ]
  }
EOF
)
