require('@nomicfoundation/hardhat-toolbox');
const fs = require('fs');
const path = require('path');

//Load private key - we don't use dotenv since the value does not follow key=value format
const filePath = path.join(__dirname, '..', '.env');
const PRIVATE_KEY = fs.readFileSync(filePath, 'utf8').trim();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    solidity: '0.8.19',
    networks: {
        stylus: {
            url: 'https://stylus-testnet.arbitrum.io/rpc',
            accounts: PRIVATE_KEY !== undefined ? [PRIVATE_KEY] : [],
            chainId: 23011913,
            blockConfirmations: 5,
        },
    },
    defaultNetwork: 'stylus',
};
