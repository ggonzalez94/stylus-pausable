const { ethers } = require('hardhat');
const { ADDRESS, ABI } = require('./common');

async function main() {
    const [signer] = await ethers.getSigners();
    const counter = await ethers.getContractAt(ABI, ADDRESS, signer);
    let tx, paused, number;

    number = await counter.number();
    console.log('Initial number value:', number.toString());

    paused = await counter.paused();
    console.log('Paused:', paused);

    console.log('Setting number to 42...');
    tx = await counter.setNumber(42);
    await tx.wait();

    number = await counter.number();
    console.log('Number is now:', number.toString());

    console.log('Pausing...');
    tx = await counter.pause();
    await tx.wait();

    paused = await counter.paused();
    console.log('Paused:', paused);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
