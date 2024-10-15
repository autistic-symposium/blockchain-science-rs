## 🧱💸 comparisons of flashloans on ethereum


<br>

### tl; dr

<br>

#### 🛠 we leverage [foundry](https://github.com/foundry-rs/foundry) to compare flashloans from **lending protocols on ethereum**, including deployment cost, and deployment size. 
#### 🛠 this experiment is adapted from **[@jeiwan's code](https://github.com/Jeiwan/flash-loans-comparison)**.

<br>

#### 💾 here is how the foundry test file looks:

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import "forge-std/Test.sol";
import "../src/AaveV2.sol";
import "../src/Balancer.sol";
import "../src/Euler.sol";
import "../src/UniswapV2.sol";
import "../src/UniswapV3.sol";


/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
// note: this test is not meant to be optimized for gas
// it is meant to be a easy to read test for this problem
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////


contract getFlashloansData is Test {

    ////////////////////////////////////
    // define all constants in the test
    ///////////////////////////////////

    uint32 constant SIMULATION_CUTOFF = 20;
    uint256 constant INTEREST_RATE = 0;
    uint256 constant AMOUNT = 1 ether;
    address constant WETH_ADDRESS = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    AAVE aave;
    Balancer balancer;
    Euler euler;
    UniswapV2 uniswapv2;
    UniswapV3 uniswapv3;


    ////////////////////////////////////////////////////////////////
    // simulation: create a vm storage for testing weth flashloans
    ////////////////////////////////////////////////////////////////

    function create_weth_storage(address addr) internal {
        // https://github.com/foundry-rs/forge-std/blob/master/src/StdStorage.sol#L6
        vm.store(
            WETH_ADDRESS,
            keccak256(abi.encode(addr, uint256(3))),
            bytes32(uint256(10 ether))
        );
    }


    //////////////////
    // TESTS: setup
    //////////////////

    function setUp() public {
        aave = new AAVE();
        create_weth_storage(address(aave));

        balancer = new Balancer();
        create_weth_storage(address(balancer));

        euler = new Euler();
        create_weth_storage(address(euler));

        uniswapv2 = new UniswapV2();
        create_weth_storage(address(uniswapv2));

        uniswapv3 = new UniswapV3();
        create_weth_storage(address(uniswapv3));
    }

   
    ///////////////
    // TESTS: run
    //////////////

    function testAAVE() public {
        address[] memory assets = new address[](1);
        assets[0] = WETH_ADDRESS;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = AMOUNT;

        uint256[] memory modes = new uint256[](1);
        modes[0] = INTEREST_RATE; 

        for (uint32 i; i < SIMULATION_CUTOFF; i++) {
            aave.flashLoan(assets, amounts, modes);
        }
    }

    function testBalancer() public {
        address[] memory assets = new address[](1);
        assets[0] = WETH_ADDRESS;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = AMOUNT;

        for (uint32 i; i < SIMULATION_CUTOFF; i++) {
            balancer.flashLoan(assets, amounts);
        }
    }

    function testEulerFinance() public {
        for (uint32 i; i < SIMULATION_CUTOFF; i++) {
            euler.flashLoan(AMOUNT);
        }
    }

    function testUniswapV2() public {
        for (uint32 i; i < SIMULATION_CUTOFF; i++) {
            uniswapv2.flashLoan(AMOUNT);
        }
    }
        
    function testUniswapV3() public {
        for (uint32 i; i < SIMULATION_CUTOFF; i++) {
            uniswapv3.flashLoan(AMOUNT);
        }
    }

}
```


#### 🕹 these boilerplates can be expanded for several purposes, including testing vulnerabilities or extracting mev data. this work is adapted from **[jeiwan's code](https://github.com/Jeiwan/flash-loans-comparison)**.

#### 💰protocol fees:

<br>

| protocol           | fees |
| -----------        | ----------- |
| aave               | 0.09%       |
| balancer           | free        |
| euler              | free        |
| uniswap v2         | 0.3009027%  |
| uniswap v3         | 0.05%, 0.3%, 1% fee, depending on pool        |


<br>

----

### installing deps

<br>

1. install **[foundry](https://book.getfoundry.sh/getting-started/installation)** (this will create `lib/forge-std`) and a **[solidity compiler](https://docs.soliditylang.org/en/latest/installing-solidity.html#installing-the-solidity-compiler)** (we are using **^0.8.16** in this project).


3. export an env variable foran **ethereum RPC URL** (*e.g.*, from **[infura's](https://app.infura.io/dashboard)**, **[alchemy](https://www.alchemy.com/)**, **[ankr's](https://www.ankr.com/rpc/avalanche/)**, or **your own node**):

```
> export RPC_URL=<RPC_URL>
```

<br>

---

### running the simulation

<br>

1. adjust the constants in the beginning of the test file.

2. build the contracts and run with:

```shell
>  make test 

⠢] Compiling...
[⠆] Compiling 6 files with 0.8.18
[⠰] Solc 0.8.18 finished in 961.13ms
Compiler run successful

Running 5 tests for test/testFlashloans.sol:getFlashloansData
[PASS] testAAVE() (gas: 1516951)
[PASS] testBalancer() (gas: 518202)
[PASS] testEulerFinance() (gas: 399865)
[PASS] testUniswapV2() (gas: 466843)
[PASS] testUniswapV3() (gas: 509303)
Test result: ok. 5 passed; 0 failed; finished in 6.13s

| src/AAVE.sol:AAVE contract |                 |       |        |        |         |
|----------------------------|-----------------|-------|--------|--------|---------|
| Deployment Cost            | Deployment Size |       |        |        |         |
| 290331                     | 1482            |       |        |        |         |
| Function Name              | min             | avg   | median | max    | # calls |
| executeOperation           | 24562           | 24667 | 24562  | 26662  | 20      |
| flashLoan                  | 69779           | 75882 | 69779  | 191845 | 20      |


| src/Balancer.sol:Balancer contract |                 |       |        |       |         |
|------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                    | Deployment Size |       |        |       |         |
| 247087                             | 1266            |       |        |       |         |
| Function Name                      | min             | avg   | median | max   | # calls |
| flashLoan                          | 24407           | 25067 | 24407  | 37608 | 20      |
| receiveFlashLoan                   | 4150            | 4150  | 4150   | 4150  | 20      |


| src/Euler.sol:Euler contract |                 |       |        |       |         |
|------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost              | Deployment Size |       |        |       |         |
| 187632                       | 969             |       |        |       |         |
| Function Name                | min             | avg   | median | max   | # calls |
| flashLoan                    | 18570           | 19582 | 18570  | 38812 | 20      |
| onFlashLoan                  | 3627            | 3627  | 3627   | 3627  | 20      |


| src/UniswapV2.sol:UniswapV2 contract |                 |       |        |       |         |
|--------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                      | Deployment Size |       |        |       |         |
| 171614                               | 889             |       |        |       |         |
| Function Name                        | min             | avg   | median | max   | # calls |
| flashLoan                            | 20153           | 22493 | 20153  | 66969 | 20      |
| uniswapV2Call                        | 4501            | 4501  | 4501   | 4501  | 20      |


| src/UniswapV3.sol:UniswapV3 contract |                 |       |        |       |         |
|--------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                      | Deployment Size |       |        |       |         |
| 171014                               | 886             |       |        |       |         |
| Function Name                        | min             | avg   | median | max   | # calls |
| flashLoan                            | 22899           | 24619 | 22899  | 57299 | 20      |
| uniswapV3FlashCallback               | 4348            | 4348  | 4348   | 4348  | 20      |

```

<br>


