// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import {IERC20} from "./interfaces.sol";

interface IAAVELendingPool {
    function flashLoan(
        address receiver,
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata modes,
        address onBehalfOf,
        bytes calldata params,
        uint16 referralCode
    ) external;
}

contract AAVE {

    ////////////////////////
    // define all constants 
    ////////////////////////

    address constant LENDING_POOL_ADDRESS =
        0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9;
    IAAVELendingPool constant lendingPool =
        IAAVELendingPool(LENDING_POOL_ADDRESS);

    ////////////////////////
    // run flashloan 
    ////////////////////////
    
    function flashLoan(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata modes
    ) public {
        lendingPool.flashLoan(
            address(this),
            assets,
            amounts,
            modes,
            address(this),
            "",
            0
        );
    }

    function executeOperation(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata premiums,
        address initiator,
        bytes calldata 
    ) external returns (bool) {
        if (msg.sender != LENDING_POOL_ADDRESS) revert();
        if (initiator != address(this)) revert();
        IERC20(assets[0]).approve(LENDING_POOL_ADDRESS, amounts[0] + premiums[0]);
        return true;
    }
}