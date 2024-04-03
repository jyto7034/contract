if [ -z "${contract}" ];
then contract=./artifacts/study.wasm
fi 
if [ -z "${keyname}" ];
then keyname=wallet
fi 
if [ -z "${password}" ];
then password="happil3571569!@A\n"
fi 

seid=~/go/bin/seid
code=$(printf $password | $seid tx wasm store $contract -y --from=$keyname --chain-id=atlantic-2 --gas=2400000 --fees=240000usei --broadcast-mode=block --node=https://sei-testnet-rpc.polkachu.com | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $code
admin_addr=$(printf $password |$seid keys show $keyname | grep -A 1 "address" | sed -n 's/.*address: //p')
printf "Admin addr id is %s\n" $admin_addr
addr=$(printf $password |$seid tx wasm instantiate $code '{}' --from $keyname --broadcast-mode=block --label "test" --chain-id atlantic-2 --gas=300000 --fees=30000usei --admin=$admin_addr -y --node=https://sei-testnet-rpc.polkachu.com | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed study address is %s\n" $addr