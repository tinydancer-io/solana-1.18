#! /bin/bash

content=$(curl http://localhost:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "id":1,
    "jsonrpc":"2.0",
    "method":"getLatestBlockhash",
    "params":[
      {
        "commitment":"finalized"
      }
    ]
  }
  ')
  echo "done1"
  blockhash=$(jq -r '.result.context.slot' <<<"$content")


  echo $blockhash
#
#   #echo $body
  curl http://localhost:8899 -X POST -H "Content-Type: application/json" -d @<( cat <<EOF
  {
    "id":1,
    "jsonrpc":"2.0",
    "method":"getVoteSignaturesForSlot",
    "params":[
    $blockhash,{
    "commitment": "finalized"
  }
    ]
  }
EOF
)
