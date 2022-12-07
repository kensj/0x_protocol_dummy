pragma solidity ^0.8.17;

import "https://github.com/0xProject/protocol/blob/development/contracts/zero-ex/contracts/src/IZeroEx.sol";

contract ERC20Transform is IZeroEx {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {

    }
}
