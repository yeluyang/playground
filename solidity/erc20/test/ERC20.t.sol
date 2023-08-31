// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test, console2} from "forge-std/Test.sol";
import {IERC20} from "../src/IERC20.sol";
import {ERC20} from "../src/ERC20.sol";

contract ERC20Test is Test {
    IERC20 private erc20;
    ERC20 private token;
    address private owner;

    function setUp() public {
        token = new ERC20("wtf", "wtf");
        erc20 = token;
        owner = address(this);
    }

    function test_mint(uint256 amount) public {
        token.mint(amount);
        assertEq(erc20.totalSupply(), amount);
        assertEq(erc20.balanceOf(owner), amount);
    }

    function test_burn(uint256 total, uint256 amount) public {
        vm.assume(total > 0);
        vm.assume(amount <= total);
        token.mint(total);

        token.burn(amount);
        assertEq(erc20.balanceOf(owner) + amount, total);
    }

    function testFail_burnEmpty(uint256 amount) public {
        vm.assume(amount > 0);
        token.burn(amount);
    }

    function testFail_burnOverflow(uint256 amount, uint256 delta) public {
        vm.assume(amount > 0);
        vm.assume(delta > 0);
        token.mint(amount);
        assertEq(erc20.balanceOf(owner), amount);
        token.burn(amount + delta);
    }

    function test_transfer(uint256 total, uint256 amount, address to) public {
        vm.assume(total > 0);
        vm.assume(amount <= total);
        token.mint(total);

        assertTrue(erc20.transfer(to, amount));
        assertEq(erc20.balanceOf(to), amount);
        assertEq(erc20.balanceOf(owner), total - amount);
        assertEq(erc20.balanceOf(to) + erc20.balanceOf(owner), total);
    }

    function testFail_transferNotEnoughBalance(uint256 total, uint256 amount, address to) public {
        vm.assume(amount > total);
        token.mint(total);
        erc20.transfer(to, amount);
    }

    function test_approve(uint256 amount, address to) public {
        assertTrue(erc20.approve(to, amount));
        assertEq(erc20.allowance(owner, to), amount);
    }

    function test_transferFrom(address spender, address to, uint256 total, uint256 approval, uint256 amount) public {
        vm.assume(spender != to);
        vm.assume(amount <= total);
        vm.assume(amount <= approval);

        token.mint(total);
        assertTrue(erc20.approve(spender, approval));

        vm.startPrank(spender);
        assertTrue(erc20.transferFrom(owner, to, amount));
        vm.stopPrank();
        assertEq(erc20.allowance(owner, spender), approval - amount);
        assertEq(erc20.balanceOf(owner), total - amount);
    }

    function testFail_transferFromAllowanceNotEnough(
        address spender,
        address to,
        uint256 total,
        uint256 approval,
        uint256 amount
    ) public {
        vm.assume(spender != to);
        vm.assume(amount <= total);
        vm.assume(amount > approval);

        token.mint(total);
        assertTrue(erc20.approve(spender, approval));

        vm.startPrank(spender);
        assertTrue(erc20.transferFrom(owner, to, amount));
        vm.stopPrank();
    }

    function testFail_transferFromBalanceNotEnough(
        address spender,
        address to,
        uint256 total,
        uint256 approval,
        uint256 amount
    ) public {
        vm.assume(spender != to);
        vm.assume(amount > total);
        vm.assume(amount <= approval);

        token.mint(total);
        assertTrue(erc20.approve(spender, approval));

        vm.startPrank(spender);
        assertTrue(erc20.transferFrom(owner, to, amount));
        vm.stopPrank();
    }
}
