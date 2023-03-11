// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import {IERC20} from "./interfaces.sol";

interface IEulerDToken {
    function flashLoan(uint256 amount, bytes calldata data) external;
}

contract Euler {

    ////////////////////////
    // define all constants 
    ////////////////////////
    address constant EULER_ADDRESS = 0x27182842E098f60e3D576794A5bFFb0777E025d3;
    address constant WETH_ADDRESS = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    address constant DTOKEN_ADDRESS = 0x62e28f054efc24b26A794F5C1249B6349454352C;
    IEulerDToken constant dToken = IEulerDToken(DTOKEN_ADDRESS);

    ////////////////////////
    // run flashloan 
    ////////////////////////
    function flashLoan(uint256 amount) public {
        dToken.flashLoan(amount, abi.encode(WETH_ADDRESS, amount));
    }

    function onFlashLoan(bytes memory data) external {
        if (msg.sender != EULER_ADDRESS) revert();
        (address underlying, uint256 amount) = abi.decode(
            data,
            (address, uint256)
        );
        IERC20(underlying).transfer(msg.sender, amount); // repay
    }
}