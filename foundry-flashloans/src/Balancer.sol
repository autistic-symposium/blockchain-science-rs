// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import {IERC20} from "./interfaces.sol";


interface IBalancer {
    function flashLoan(
        address recipient,
        address[] memory tokens,
        uint256[] memory amounts,
        bytes memory userData
    ) external;
}


contract Balancer {

    ////////////////////////
    // define all constants 
    ////////////////////////

    address constant BALANCER_ADDRESS =
        0xBA12222222228d8Ba445958a75a0704d566BF2C8;
    IBalancer constant balancer = IBalancer(BALANCER_ADDRESS);

    ////////////////////////
    // run flashloan 
    ////////////////////////
    
    function flashLoan(address[] calldata tokens, uint256[] calldata amounts)
        public
    {
        balancer.flashLoan(address(this), tokens, amounts, "");
    }

    function receiveFlashLoan(
        IERC20[] calldata tokens,
        uint256[] calldata amounts,
        uint256[] calldata, /* feeAmounts */
        bytes calldata /* userData */
    ) public payable {
        if (msg.sender != BALANCER_ADDRESS) revert();
        tokens[0].transfer(BALANCER_ADDRESS, amounts[0]);
    }
}