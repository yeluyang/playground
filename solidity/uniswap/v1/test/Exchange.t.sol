// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {Test, console2} from "forge-std/Test.sol";

import {Token} from "../src/Token.sol";
import {Exchange} from "../src/Exchange.sol";

abstract contract TestExchangeBase {
    IERC20 public token;
    Exchange public exchange;
}

contract ExchangeTest is Test, TestExchangeBase {
    uint256 public constant tokenSupply = 100000 ether;
    uint256 constant tokens = 2000 ether;
    uint256 constant ethers = 1000 ether;
    uint256 _lpTokens;

    receive() external payable {}

    function setUp() public {
        token = new Token("zuniswap", "Z", tokenSupply);
        exchange = new Exchange(address(token));
        assertTrue(token.approve(address(exchange), tokenSupply));
        _lpTokens = exchange.addLiquidity{value: ethers}(tokens);
    }

    function test_removeLiquidity() public {
        exchange.addLiquidity{value: 1 ether}(100 ether);
        exchange.ethToTokenSwap{value: 1 ether}(0);
        (uint256 ethReturned, uint256 tokenReturned) = exchange.removeLiquidity(_lpTokens);
        assertEq(ethReturned, ethers);
        assertEq(tokenReturned, tokens);
    }

    function test_getReserve() public {
        assertEq(address(exchange).balance, ethers);
        assertEq(exchange.getReserve(), tokens);
        assertEq(exchange.getReserve(), token.balanceOf(address(exchange)));
    }

    function test_getPrice() public {
        assertEq(exchange.getPrice(ethers, tokens), 500);
        assertEq(exchange.getPrice(tokens, ethers), 2000);
    }

    function test_getTokenAmount() public {
        assertEq(exchange.getTokenAmount(1 ether), 1998001998001998001 wei); // 1.99 eth
        assertEq(exchange.getTokenAmount(100 ether), 181818181818181818181 wei); // 181 eth
        assertEq(exchange.getTokenAmount(1000 ether), 1000 ether);
    }

    function test_getEthAmount() public {
        assertEq(exchange.getEthAmount(2 ether), 999000999000999000 wei); // 0.99 eth
        assertEq(exchange.getEthAmount(100 ether), 47619047619047619047 wei); // 47 eth
        assertEq(exchange.getEthAmount(2000 ether), 500 ether);
    }

    function test_ethToTokenSwap() public {
        uint256 lastETHBalance = address(this).balance;
        uint256 lastTokenBalance = token.balanceOf(address(this));

        exchange.ethToTokenSwap{value: 1 ether}(0);

        assertEq(address(this).balance, lastETHBalance - 1 ether, "eth of tarder");
        assertEq(token.balanceOf(address(this)), lastTokenBalance + 1998001998001998001 wei, "token of tarder");
    }

    function test_tokenToEthSwap() public {
        uint256 lastETHBalance = address(this).balance;
        uint256 lastTokenBalance = token.balanceOf(address(this));

        exchange.tokenToEthSwap(2 ether, 0);

        assertEq(address(this).balance, lastETHBalance + 999000999000999000 wei, "eth of tarder");
        assertEq(token.balanceOf(address(this)), lastTokenBalance - 2 ether, "token of tarder");
    }
}
