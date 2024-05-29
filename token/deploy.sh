

seid=~/go/bin/seid

if [ -z "${pass}" ];
then pass="happil3571569!@A\n"
fi

code=$(printf $pass | $seid tx wasm store ./artifacts/token.wasm -y --from=mainnet-wallet --chain-id=pacific-1 --broadcast-mode=block --gas=2400000 --fees=48000usei --node=https://sei-rpc.polkachu.com/ | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $code
addr=$(printf $pass | $seid tx wasm instantiate $code '{}' --from=mainnet-wallet --broadcast-mode=block --label "EGG" --chain-id=pacific-1 --gas=300000 --fees=30000usei --admin=sei1vrcc6zjy08s63r88wkvvssp7pm274hfx2u9xqk -y --node=https://sei-rpc.polkachu.com | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed study address is %s\n" $addr

# '{"create_transaction": {"seller": "sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4","desired_item": "nft","nft_token_id": "1005"}}'
# '{"approve_transaction": {"buyer": "sei17k3ah8kd2k6u4prlsvm8f3psghw2at7cf2mjpr"}}'
# --node=https://rpc.atlantic-2.seinetwork.io -y --broadcast-mode=block --gas=500000 --fees=50000usei --chain-id=atlantic-2 --from=wallet
