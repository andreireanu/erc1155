# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/erc1155/wallets/alice.pem"
WALLET_BOB="${PWD}/erc1155/wallets/bob.pem"
# SC ADDRESS WITH LOCAL MINT:
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqnkgxk3j9427v232kwx9sqmgsr6d8jt76y8dqfyrujn"
CONTRACT_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${CONTRACT_ADDRESS})"
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
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable
}

upgrade() {
 mxpy contract upgrade ${CONTRACT_ADDRESS} \
    --pem="erc1155/wallets/bob.pem" \
    --chain=${CHAIN_ID} \
    --proxy=${PROXY} \
    --recall-nonce \
    --bytecode=erc1155/output/erc1155.wasm \
    --gas-limit=80000000 \
    --send \
    --metadata-payable
}



### ISSUE, MINT, ROLES

TKN_NAME="METAL"
TKN_TICKER="METAL"
AMOUNT=10000

mintFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=140000000 \
    --function="mintFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $AMOUNT
} 

NFT_NAME="ATHENASWORD"
NFT_TICKER="SWD"
 
issueNonFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=140000000 \
    --function="issueNonFungibleToken" \
    --arguments "str:"$NFT_NAME "str:"$NFT_TICKER
}


mintNft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=5500000 \
    --function="mintNft"
} 
 
 
DEPOSIT_TOKEN=GOLD-ed11cc
DEPOSIT_SUPPLY=5
DEPOSIT_NONCE=0
DEPOSIT_FUNCTION="depositToken"
DEPOSIT_TOKEN_DUMMY=GOLD-000000
DEPOSIT_SUPPLY_DUMMY=2
DEPOSIT_NONCE_DUMMY=100

depositToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=100000000 \
    --function="ESDTTransfer" \
    --arguments "str:"${DEPOSIT_TOKEN} ${DEPOSIT_SUPPLY} "str:"${DEPOSIT_FUNCTION} ${DEPOSIT_SUPPLY} ${DEPOSIT_NONCE} "str:"${DEPOSIT_TOKEN}
} 


DEPOSIT_NFT=SLM-a81055
DEPOSIT_NFT_NONCE=1
DEPOSIT_NFT_SUPPLY=1

depositNFTToken() {
    mxpy --verbose contract call ${ALICE_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="ESDTNFTTransfer" \
    --arguments "str:"${DEPOSIT_NFT} ${NFT_NONCE} ${DEPOSIT_NFT_SUPPLY} ${CONTRACT_ADDRESS} "str:"${DEPOSIT_FUNCTION} ${DEPOSIT_NFT_SUPPLY} ${DEPOSIT_NFT_NONCE} "str:"${DEPOSIT_NFT}
} 

SUPPLY=1
NONCE=1
TOKEN=SLM-a81055

withdrawToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="withdrawToken" \
    --arguments ${SUPPLY} ${NONCE} "str:"${TOKEN}
}  

### ERC1155

ID_1=1
ID_2=2
ID_3=3
ID_4=4
ID_5=5
ID_6=6
ID_7=7

balanceOf() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="balanceOf" \
    --arguments ${BOB_ADDRESS} ${ID_1}  
}

balanceOfBatch() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="balanceOfBatch" \
    --arguments ${BOB_ADDRESS} ${ID_1} ${ALICE_ADDRESS} ${ID_3} 
}

APPROVAL=1
 
setApprovalForAll() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=10000000 \
    --function="setApprovalForAll" \
    --arguments ${BOB_ADDRESS} ${APPROVAL}
} 

isApprovedForAll() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="isApprovedForAll" \
    --arguments ${ALICE_ADDRESS} ${BOB_ADDRESS}
}

TRANSFER_AMOUNT=2000

safeBatchTransferFrom() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=10000000 \
    --function="safeBatchTransferFrom" \
    --arguments ${BOB_ADDRESS} ${ALICE_ADDRESS} ${ID_7} ${TRANSFER_AMOUNT}
}


### GETS

balanceOfBatch() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="balanceOfBatch" \
    --arguments ${BOB_ADDRESS} ${ID_1} ${ALICE_ADDRESS} ${ID_3} 
}

ID=7

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
    --arguments ${BOB_ADDRESS_HEXX} 
}
 
getCurrentIssuedNft() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCurrentIssuedNft"
}
 
getId() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getId" \
    --arguments "str:"${TOKEN} 
}

getAddress_1() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAddress" \
    --arguments $ID_1
}
 
getAddress_4() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAddress" \
    --arguments $ID_4
}

getBalanceAlice() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBalance" \
    --arguments ${ALICE_ADDRESS_HEXX} 
}

getBalanceBob() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBalance" \
    --arguments ${BOB_ADDRESS_HEXX} 
}

### DEV CALLS (HANDLE WITH CARE)
 
initTokenCount() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
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
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=140000000 \
    --function="addToStorage" \
    --arguments "str:"$EXISTING_TOKEN  
}

NFT_ISSUE_NAME=THM-b30377
 
setLocalRoles() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/bob.pem" \
    --gas-limit=140000000 \
    --function="setLocalRoles" \
    --arguments "str:"$NFT_ISSUE_NAME  
}


 