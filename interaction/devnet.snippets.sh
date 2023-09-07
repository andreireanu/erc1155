# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/erc1155/wallets/alice.pem"
WALLET_BOB="${PWD}/erc1155/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqmm5gavp332jtadaqzvv0xyk0s7s7hm387wpqg3506p"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"

### MAIN

deploy() {
 mxpy contract deploy --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --bytecode=erc1155/output/erc1155.wasm \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable
}

upgrade() {
 mxpy contract upgrade ${CONTRACT_ADDRESS} \
    --pem="erc1155/wallets/alice.pem" \
    --chain=${CHAIN_ID} \
    --proxy=${PROXY} \
    --recall-nonce \
    --bytecode=erc1155/output/erc1155.wasm \
    --gas-limit=60000000 \
    --send \
    --metadata-payable
}



### ISSUE, MINT, ROLES

TKN_NAME="GOLD"
TKN_TICKER="GOLD"
AMOUNT=1000

mintFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="mintFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $AMOUNT
} 

NFT_NAME="THORHAMMER"
NFT_TICKER="THM"
 
issueNonFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="issueNonFungibleToken" \
    --arguments "str:"$NFT_NAME "str:"$NFT_TICKER
}

NFT_ISSUE_NAME=THM-b30377

setLocalRoles() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="setLocalRoles" \
    --arguments "str:"$NFT_ISSUE_NAME  
}

NFT_ID=2
NFT_NAME="THORHAMMER"
URI="https://ipfs.io/ipfs/QmTSzERByLKRLA2YGK5sAuFGmNktYkrKcXRr7SsrsqzQG7"
ATTR="Hammer:Best"

setLocalRoles() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="setLocalRoles" \
    --arguments "str:"$NFT_ISSUE_NAME  
}

createNft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=5500000 \
    --function="createNft" \
    --arguments "str:"$NFT_ID "str:"$NFT_NAME "str:"$URI "str:"$ATTR 
} 
 
 

### GETS

ID=1

getTokenCount() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokenCount"  
}

getAddress() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAddress" \
    --arguments $ID
}

getTokenName() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokenName" \
    --arguments $ID
}

getBalance() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBalance" \
    --arguments ${ALICE_ADDRESS_HEXX} 
}
 
### DEV CALLS (HANDLE WITH CARE)
 
initTokenCount() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=3000000 \
    --function="initTokenCount"
}

EXISTING_TOKEN=ERC3-26897c

addToStorage() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="addToStorage" \
    --arguments "str:"$EXISTING_TOKEN  
}




 