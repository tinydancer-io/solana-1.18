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
      "votePubkey": ["3ANJb42D3pkVtntgT6VtW2cD3icGVyoHi2NGwtXYHQAs","J7v9ndmcoBuo9to2MnHegLnBkC9x3SAVbQBJo5MMJrN1"],
      "commitment":"confirmed"
    } 
  ]
  }
EOF
) | jq
