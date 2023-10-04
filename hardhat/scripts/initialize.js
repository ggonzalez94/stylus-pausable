const { ethers } = require('hardhat');
const { ABI, ADDRESS } = require('./common');

async function main() {
    const [signer] = await ethers.getSigners();
    console.log('Signer:', signer.address);
    console.log(ABI);
    console.log(ADDRESS);

    const counter = await ethers.getContractAt(ABI, ADDRESS, signer);
    console.log(counter);
    let tx, owner;

    owner = await counter.owner();
    console.log('Initial owner:', owner);

    console.log('Initializing contract...');
    tx = await counter.initialize(signer.address);
    await tx.wait();

    owner = await counter.owner();
    console.log('New owner:', owner);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
