ALICE="${USERS}/alice.pem"
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)
DEPLOY_ARGUMENTS=""
DEPLOY_GAS="80000000"
deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS}  \
          --outfile="deploy.json" --send

    TRANSACTION=$(erdpy data parse --file="deploy.json" --expression="data['result']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address --value=${ADDRESS}
    erdpy data store --key=deployTransaction --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --outfile="simulate.json" --simulate

    TRANSACTION=$(erdpy data parse --file="simulate.json" --expression="data['result']['hash']")
    ADDRESS=$(erdpy data parse --file="simulate.json" --expression="data['emitted_tx']['address']")
    RETCODE=$(erdpy data parse --file="simulate.json" --expression="data['result']['returnCode']")
    RETMSG=$(erdpy data parse --file="simulate.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    erdpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    erdpy account get --address=$ADDRESS --omit-fields="['code']"
}

status() {
    erdpy --verbose contract query ${ADDRESS} --function="checkTask"
}

getTaskContent() {
    erdpy --verbose contract query ${ADDRESS} --function="getTaskContent" --arguments 0001
}

addTask() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="addTask" --arguments 2 --arguments Helooo
}
