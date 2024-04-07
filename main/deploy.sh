

seid=~/go/bin/seid
code=$($seid tx wasm store ./artifacts/main.wasm -y --from=wallet --chain-id=atlantic-2 --broadcast-mode=block --gas=2400000 --fees=48000usei --node=https://sei-testnet-rpc.polkachu.com/ | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $code
addr=$($seid tx wasm instantiate $code '{"token_address":"factory/sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4/test", "nft_contract_address":"sei1mwwguwrljgrmtz8f823cd3qf0sx0mgecgzdxzg3r2are6w8fqzlqrwaakd", "exchange_rate":1, "expiration_block":2000}' --from=wallet --broadcast-mode=block --label "STUDY1" --chain-id atlantic-2 --gas=300000 --fees=30000usei --admin=sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4 -y --node=https://sei-testnet-rpc.polkachu.com | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed study address is %s\n" $addr

# '{"create_transaction": {"seller": "sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4","desired_item": "nft","nft_token_id": "1005"}}'
# '{"approve_transaction": {"buyer": "sei17k3ah8kd2k6u4prlsvm8f3psghw2at7cf2mjpr"}}'
# --node=https://rpc.atlantic-2.seinetwork.io -y --broadcast-mode=block --gas=500000 --fees=50000usei --chain-id=atlantic-2 --from=wallet