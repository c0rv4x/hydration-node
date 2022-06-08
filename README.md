# HydraDX node

CROSS-CHAIN LIQUIDITY PROTOCOL BUILT ON SUBSTRATE

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md).
We are welcoming and friendly community please follow our [Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Local Development

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Simple Setup

Install all the required dependencies with a single command (be patient, this can take up to 30
minutes).

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### Manual Setup

Find manual setup instructions at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started/#manual-installation).

### Build

Once the development environment is set up, build the node. This command will build the
[Wasm](https://substrate.dev/docs/en/knowledgebase/advanced/executor#wasm-execution) and
[native](https://substrate.dev/docs/en/knowledgebase/advanced/executor#native-execution) code:

```bash
cargo build --release
```

## Run

### Local Testnet

Relay chain repository (polkadot) has to be built in `../polkadot`
Install `polkadot-launch` utility used to start network.

```
npm install -g polkadot-launch
```

Start local testnet with 4 relay chain validators and HydraDX as a parachain with 2 collators.

```
cd ./rococo-local
polkadot-launch config.json
```

Observe HydraDX logs

```
multitail 99*.log
```

### Use testing runtime

In the case of starting a testnet using the `polkadot-launch` tool, 
we don't have an option to communicate to its internal commands that we would like to use the testing runtime.
To overcome this limitation, rename the binary so it starts with the `testing` prefix, e.g. `testing-hydradx`.
Such a binary always uses the testing runtime, even if the `--runtime testing` option is not specified.

Start local testnet with testing runtime
```
cd ./rococo-local
polkadot-launch testing-config.json
```

### Interaction with the node

Go to the polkadot apps at https://dotapps.io

Then open settings screen -> developer and paste

*NOTE - FixedU128 type is not yet implemented for polkadot apps. Balance is a measure so price can be reasonably selected. If using polkadot apps to create pool:*
- 1 Mega Units equals 1:1 price
- 20 Mega Units equals 20:1 price
- 50 Kilo Units equals 0.05:1 price

```
{
  "types": [
    {
      "AssetPair": {
        "asset_in": "AssetId",
        "asset_out": "AssetId"
      },
      "Amount": "i128",
      "AmountOf": "Amount",
      "Address": "AccountId",
      "OrmlAccountData": {
        "free": "Balance",
        "frozen": "Balance",
        "reserved": "Balance"
      },
      "BalanceInfo": {
        "amount": "Balance",
        "assetId": "AssetId"
      },
      "Chain": {
        "genesisHash": "Vec<u8>",
        "lastBlockHash": "Vec<u8>"
      },
      "CurrencyId": "AssetId",
      "CurrencyIdOf": "AssetId",
      "Intention": {
        "who": "AccountId",
        "asset_sell": "AssetId",
        "asset_buy": "AssetId",
        "amount": "Balance",
        "discount": "bool",
        "sell_or_buy": "IntentionType"
      },
      "IntentionId": "Hash",
      "IntentionType": {
        "_enum": [
          "SELL",
          "BUY"
        ]
      },
      "LookupSource": "AccountId",
      "OrderedSet": "Vec<AssetId>",
      "Price": "Balance",
      "Fee": {
        "numerator": "u32",
        "denominator": "u32"
      }
    }
  ],
  "alias": {
    "tokens": {
      "AccountData": "OrmlAccountData"
    }
  }
}
```

Connect to the
- Hacknet: `wss://hack.hydradx.io:9944`
- [Stakenet](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc-01.snakenet.hydradx.io): `wss://rpc-01.snakenet.hydradx.io`
- or local node – if you are on chromium based browser, set chrome://flags/#allow-insecure-localhost

### Performance check

Prerequisites: rust/cargo, python 3.8+

With the following script it is possible to run a simple performance check. It might be useful
to determine whether your machine is suitable to run HydraDX node.

From the top-level node directory:

```bash
./scripts/check_performance.sh
```

This will run series of benchmarks ( which may take a while).
The output will show benchmark results of HydraDX pallets and comparison against reference values.

The most interesting information would be the difference between the HydraDx benchmark value and the local machine's benchmark.

If the difference is >= 0, performance is similar or better.
However, if the difference < 0 - your machine might not suitable to run HydraDX node. Contact HydraDX devs to discuss the results.

### Running a stakenet node

```bash
./target/release/hydra-dx --chain lerna
```

### Honorable contributions
[@apopiak](https://github.com/apopiak) for great reviews [#87](https://github.com/galacticcouncil/HydraDX-node/pull/87) and support.
