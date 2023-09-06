# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/erc1155/wallets/alice.pem"
WALLET_BOB="${PWD}/erc1155/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqlqwfcxwwy95tkh3yrxhkjegww6mvxtvh7wpqqd5w4y"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"

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

setLocalRoles() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="setLocalRoles"
}





















######## CONTRACT CALLS

TKN_NAME="TestTokenName"
TKN_TICKER="TTN"
NR=1000000

issueFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="issueFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $NR 
} 

issueNonFungibleToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="issueNonFungibleToken" \
    --arguments "str:"$ALICE_ADDRESS "str:"$TKN_TICKER  
}  

NFT_NAME="Campaign1"
URI="ipfs://bafybeihbh6yup53quguma2qze3o7v6m6cef3cvu3qlap3jvc5lzymnmo7i"
NFTS_NR=10
ATTR="Band:Best"
PRICE=100

createNft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=5500000 \
    --function="createNft" \
    --arguments "str:"$NFT_NAME $NFTS_NR "str:"$URI "str:"$ATTR $PRICE
} 

CAMPAIGN_NAME="TestCampaign"
HASHTAG="DIMITRISBATS"
AMOUNT=10000

createCampaign() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=5500000 \
    --function="createCampaign" \
    --arguments "str:"$CAMPAIGN_NAME "str:"$HASHTAG $AMOUNT   
}

NR_1=1000000
NR_2=2
NR_3=3
HASHTAG=carlasdreams

sendCampaignTokens() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=5500000 \
    --function="sendCampaignTokens" \
    --arguments "str:"$HASHTAG ${ALICE_ADDRESS_HEXX} $NR_1 ${BOB_ADDRESS_HEXX} $NR_2 ${MARTA_ADDRESS_HEXX} $NR_3    
}
 
######## QUERRIES

getCreatorToken() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCreatorToken" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  

getCreatorNft() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCreatorNft" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  
 
getCampaigns() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCampaigns" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }   

NFT=CARLA-a7922e
IDX=3

getNftPrices() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftPrices" \
    --arguments "str:"${NFT}
    }   


getNftPrice() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftPrice" \
    --arguments "str:"${NFT} $IDX
    }   


getPaymentToken() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getPaymentToken" \
    --arguments "str:"${NFT}  
    }   



######## CLEAR

clearToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=6000000 \
    --function="clearToken" \
    --arguments ${ALICE_ADDRESS_HEXX}
} 

clearNft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=6000000 \
    --function="clearNft" \
    --arguments ${ALICE_ADDRESS_HEXX}
} 

TOKEN=

clearPaymentToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=6000000 \
    --function="clearPaymentToken" \
    --arguments "str:"$TOKEN
} 

clearCampaign() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=6000000 \
    --function="clearCampaign" \
    --arguments ${ALICE_ADDRESS_HEXX}
} 