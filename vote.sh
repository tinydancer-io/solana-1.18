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
    "method":"getVoteSignatures",
    "params":[
    $blockhash,
    { 
      "votePubkey": ["Ninja1spj6n9t5hVYgF3PdnYz2PLnkt7rvaw3firmjs"],
      "commitment":"confirmed"
    } 
  ]
  }
EOF
)
