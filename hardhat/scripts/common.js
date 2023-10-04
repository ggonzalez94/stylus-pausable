exports.ABI = [
    'function paused() external view returns (bool)',
    'function renounceOwnership() external',
    'function transferOwnership(address new_owner) external',
    'function owner() external view returns (address)',
    'function initialize(address initial_owner) external',
    'function number() external view returns (uint256)',
    'function setNumber(uint256 new_number) external',
    'function increment() external',
    'function pause() external',
    'function unpause() external',
];

// const ABI = [
//     'function number() external view returns (uint256)',
//     'function setNumber(uint256 new_number) external',
//     'function increment() external',
//     'function pause() external',
//     'function unpause() external',
//     'function owner() external returns (address)',
//     'function transferOwnership(address new_owner) external returns (bool)',
//     'function renounceOwnership() external returns (bool)',
//     'function paused() external view returns (bool)',
// ];

exports.ADDRESS = '0x5d1bd020dc0b414a82feb81c23eff58624db79c3';
