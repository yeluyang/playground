// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test, console2} from "forge-std/Test.sol";
import {ERC20} from "../src/ERC20.sol";
import {Faucet} from "../src/faucet.sol";

contract FaucetTest is Test {
    ERC20 public token;
    Faucet public faucet;

    function setUp() public {
        token = new ERC20("wtf", "wtf");
        faucet = new Faucet(address(token));
    }

    function test_requestTokens(uint256 total, address fresh) public {
        vm.assume(total >= faucet.amountAllowed());
        token.mint(total);
        token.transfer(address(faucet), total);

        vm.startPrank(fresh);
        faucet.requestTokens();
        vm.stopPrank();
        assertTrue(faucet.requestedAddress(fresh));
    }

    function test_requestTokens_BalanceNotEnough(uint256 total, address fresh) public {
        vm.assume(total < faucet.amountAllowed());
        token.mint(total);
        token.transfer(address(faucet), total);

        vm.startPrank(fresh);
        vm.expectRevert(abi.encodeWithSelector(Faucet.ErrBalanceNotEnough.selector, address(faucet), total));
        faucet.requestTokens();
        vm.stopPrank();
    }

    function test_requestTokens_AlreadyRequested(uint256 total, address fresh) public {
        vm.assume(total >= faucet.amountAllowed());
        token.mint(total);
        token.transfer(address(faucet), total);

        vm.startPrank(fresh);
        faucet.requestTokens();
        vm.expectRevert(abi.encodeWithSelector(Faucet.ErrAlreadyRequested.selector, fresh));
        faucet.requestTokens();
        vm.stopPrank();
    }
}
