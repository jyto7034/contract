

seid=~/go/bin/seid
code=$($seid tx wasm store ./artifacts/main.wasm -y --from=wallet --chain-id=atlantic-2 --broadcast-mode=block --gas=2400000 --fees=48000usei --node=https://sei-testnet-rpc.polkachu.com/ | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $code
addr=$($seid tx wasm instantiate $code '{"token_address":"factory/sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4/test", "recipient":"sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4", "product":"token"}' --from=wallet --broadcast-mode=block --label "STUDY1" --chain-id atlantic-2 --gas=300000 --fees=30000usei --admin=sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4 -y --node=https://sei-testnet-rpc.polkachu.com | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed study address is %s\n" $addr