[![codecov](https://codecov.io/gh/Anchor-Protocol/anchor-bEth-contracts/branch/main/graph/badge.svg?token=1EGN3Z1YDN)](https://codecov.io/gh/Anchor-Protocol/anchor-bEth-contracts)

# Anchor bEth Contracts

This monorepository contains the source code for the smart contracts implementing bEth on the [Terra](https://terra.money) blockchain.

You can find information about the architecture, usage, and function of the smart contracts on the official Anchor documentation [site](https://anchorprotocol.com/).
## Development

### Environment Setup

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit / Integration Tests

Each contract contains Rust unit tests embedded within the contract source directories. You can run:

```sh
cargo test unit-test
cargo test integration-test
```

### Compiling

After making sure tests pass, you can compile each contract with the following:

```sh
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

#### Production

For production builds, run the following:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.11.5
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

## License

Copyright 2021 Anchor Protocol

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0. Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

See the License for the specific language governing permissions and limitations under the License.


## Updated Instructions

To build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.16.1
```

To test (need custom cosmes with multisig enabled)
```
import { MsgStoreCode, MsgMigrateContract, RpcClient, simulateTx, Tx, getAccount, toBaseAccount, Secp256k1PubKey, MsgSend } from "cosmes/client";
import { CosmosCryptoMultisigLegacyAminoPubKey, CosmwasmWasmV1QueryCodesService } from "cosmes/protobufs";
import { UnsignedTx } from "cosmes/wallet";
import { readFileSync }  from 'fs';

async function main() {
    const deployerWallet = "terra1gufrav46pnpwf03yu7xz76ylkmatsxtplrxnmc";
    const contractAddress = "terra10cxuzggyvvv44magvrh3thpdnk9cmlgk93gmx2"
    const chainId = "columbus-5"
    const wasmCode = readFileSync("../anchor_beth_converter.wasm")

    const codes = await RpcClient.query('https://terra-classic-rpc.publicnode.com:443', CosmwasmWasmV1QueryCodesService, {
        pagination: {
            reverse: true,
            limit: BigInt(1),
        }
    })
    const codeId = codes.codeInfos[0].codeId

    const unsignedTx: UnsignedTx = {
        msgs: [
            new MsgStoreCode({
                sender: deployerWallet,
                wasmByteCode: wasmCode,
                instantiatePermission: {
                    permission: 3,
                    addresses: []
                },
            }),
            new MsgMigrateContract({
                sender: deployerWallet,
                codeId: codeId + BigInt(1),
                contract: contractAddress,
                msg: new Uint8Array(0),
            }),
        ]
      }; 
    const account = await getAccount('https://terra-classic-rpc.publicnode.com:443', {
        address: deployerWallet
    })
    const baseAccount = toBaseAccount(account);
    // console.log(baseAccount);
    const publicKey = CosmosCryptoMultisigLegacyAminoPubKey.fromBinary(baseAccount.pubKey!.value);
    // console.log(publicKey);

    const { gasInfo, result } = await simulateTx('https://terra-classic-rpc.publicnode.com:443', {
        sequence: baseAccount.sequence,
        tx: new Tx({ chainId: chainId, pubKey: new Secp256k1PubKey({
            chainId,
            key: new Uint8Array()
        }),msgs: unsignedTx.msgs }),
      });
    console.log(JSON.stringify(result?.events));
      
      
}

main();
```

Custom cosmes
node_modules/cosmes/src/client/models/Tx.ts
```
  private getSignerInfo(
    sequence: bigint,
    mode: ProtoSignMode
  ): PlainMessage<ProtoSignerInfo> {
    return {
      publicKey: toAny(this.data.pubKey.toProto()),
      sequence: sequence,
      modeInfo: {
        sum: {
          case: "multi",
          value: {
            modeInfos: []
          },
        },
      },
    };
  }
```