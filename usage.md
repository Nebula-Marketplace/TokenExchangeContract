# Instantiation
To instantiate, use code 2923 on testnet, and [MAINNET] on mainnet. Here is an example instantiation message:
```js
{
    "denom":"factory/inj1d0zfq42409a5mhdagjutl8u6u9rgcm4h8zfmfq/nbla",
    "fee":"100000000",
    "symbol":"NBLA", 
    "decimals":6, // Cw20 token decimals for fee computation
    "name":"Nebula"
}
```

```ts
import {
    MsgExecuteContract,
    MsgBroadcasterWithPk,
} from "@injectivelabs/sdk-ts";
import { Network } from "@injectivelabs/networks";
  
const injectiveAddress = "wallet";
const contractAddress = "contract";

async function list() {
    const msg = MsgExecuteContract.fromJSON({
    contractAddress,
    sender: injectiveAddress,
    funds: [
        {
            denom: "fully/qualified/token_denom",
            amount: (1*(10**6)).toString(),
        },
    ],
    msg: {
        "List": {
            "price": (0.0001*(10**18)).toString(), // price per token. in this case, 0.0001 INJ 
        }
    }
    });

    const txHash = await new MsgBroadcasterWithPk({
        privateKey: "B86009E44772A62C8C548E44C51972913F12E7419B8A4CC1DC42A4A3BCAE8FB4",
        network: Network.Testnet,
    }).broadcast({
        msgs: msg,
        injectiveAddress: injectiveAddress,
    });

    console.log(txHash);
}

async function buy() {
    const msg = MsgExecuteContract.fromJSON({
        contractAddress,
        sender: injectiveAddress,
        funds: [
            {
                denom: "inj",
                amount: (3*(10**18)).toString(),
            },
        ],
        msg: {
            "Buy": {
                "amount": (1*(10**5)).toString(), // 0.1 of token w/ 6 decimals
            }
        }
    });

    const txHash = await new MsgBroadcasterWithPk({
        privateKey: "privateKey",
        network: Network.Testnet,
    }).broadcast({
        msgs: msg,
        injectiveAddress: injectiveAddress,
    });

    console.log(txHash);
}

// list()
buy()
```
