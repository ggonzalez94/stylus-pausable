# Stylus Pausable

Sample implementation of OpenZeppelin Pausable contract in Stylus using Rust. The codebase also includes an implementation for Ownable to allow only the owner to pause and unpause the smart contract.

## Installation

In order to install the rust toolchain, including cargo and all the necessary Stylus dependencies, follow the official guide [here](https://docs.arbitrum.io/stylus/stylus-quickstart#prerequisites). Also, don't forget to bridge some funds to the Stylus testnet.

## Deployment

1. Create a `.env` file in the root directory of the project and add the Private Key for the account you want to deploy the contract from. The file should contain the raw PK without any variable name(since that's the format that the stylus toolchain uses). The hardhat script is also configured to read from that same file.

2. Check if your program will successfully deploy and activate onchain by running:

```bash
$ cargo stylus check
```

3. Deploy and activate the program onchain by running:

```bash
$ cargo stylus deploy --private-key-path=./.env
```

As of now there's no way to do source code verification on the block explorer, so you will have to export the abi and interact with the contract using ethers or similar.

4. Export the abi by running:

```bash
$ cargo stylus export-abi
```

5. Now change directory to the hardhat folder where the scripts reside and run any of the provided scripts to interact with the contract, feel free to customize them or create your own. Also make sure you specify the address of your deployed contract in the `common.js` [file](./hardhat/scripts/common.js).

```bash
$ yarn hardhat run scripts/<yourScript.js>
```
