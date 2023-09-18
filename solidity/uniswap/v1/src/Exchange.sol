// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.0;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IFactory} from "./IFactory.sol";

contract Exchange is ERC20 {
    address public tokenAddress;
    address public factoryAddress;

    constructor(address _token) ERC20("Uniswap_v1_LP-Tokens", "V1LP") {
        require(_token != address(0), "invalid token address");

        tokenAddress = _token;
        factoryAddress = msg.sender;
    }

    function addLiquidity(uint256 _dx) public payable returns (uint256) {
        IERC20 token = IERC20(tokenAddress);
        uint256 x = getReserve();
        if (x == 0) {
            token.transferFrom(msg.sender, address(this), _dx);
            uint256 lptokens = address(this).balance;
            _mint(msg.sender, lptokens);
            return lptokens;
        } else {
            uint256 y = address(this).balance - msg.value;
            // (x + dx) / (y + dy) = x / y
            // xy + ydx = xy + xdy
            // dx / dy = x / y
            // dx = dy * x / y
            uint256 dx = msg.value * x / y; // msg.value <=> dy
            require(_dx >= dx, "insufficient token amount");
            token.transferFrom(msg.sender, address(this), dx);

            uint256 lptokens = totalSupply() * msg.value / y;
            _mint(msg.sender, lptokens);
            return lptokens;
        }
    }

    function removeLiquidity(uint256 _amount) public returns (uint256, uint256) {
        require(_amount > 0, "invalid amount");

        uint256 ethAmount = _amount * address(this).balance / totalSupply();
        uint256 tokenAmount = _amount * getReserve() / totalSupply();

        _burn(msg.sender, _amount);
        payable(msg.sender).transfer(ethAmount);
        IERC20(tokenAddress).transfer(msg.sender, tokenAmount);

        return (ethAmount, tokenAmount);
    }

    function getReserve() public view returns (uint256) {
        return IERC20(tokenAddress).balanceOf(address(this));
    }

    function getPrice(uint256 x, uint256 y) public pure returns (uint256) {
        require(x > 0 && y > 0, "invalid reserves");

        return (x * 1000) / y;
    }

    function getAmount(uint256 _dx, uint256 x, uint256 y) private pure returns (uint256) {
        require(x > 0 && y > 0, "invalid reserves");

        // fee = 1%. so, dx = dx * 0.99, equals to dx * (100 - 1) / 100
        return _dx * 99 * y / (x * 100 + _dx * 99);
    }

    function getTokenAmount(uint256 _ethSold) public view returns (uint256) {
        require(_ethSold > 0, "ethSold is too small");

        return getAmount(_ethSold, address(this).balance, getReserve());
    }

    function getEthAmount(uint256 _tokenSold) public view returns (uint256) {
        require(_tokenSold > 0, "tokenSold is too small");

        return getAmount(_tokenSold, getReserve(), address(this).balance);
    }

    function ethToToken(uint256 _minTokens, address recipient) public payable {
        uint256 tokensBought = getAmount(msg.value, address(this).balance - msg.value, getReserve());

        require(tokensBought >= _minTokens, "insufficient output amount");

        IERC20(tokenAddress).transfer(recipient, tokensBought);
    }

    function ethToTokenSwap(uint256 _minTokens) public payable {
        ethToToken(_minTokens, msg.sender);
    }

    function tokenToEthSwap(uint256 _tokensSold, uint256 _minEths) public {
        uint256 ethsBought = getAmount(_tokensSold, getReserve(), address(this).balance);

        require(ethsBought >= _minEths, "insufficient output amount");

        IERC20(tokenAddress).transferFrom(msg.sender, address(this), _tokensSold);
        payable(msg.sender).transfer(ethsBought);
    }

    function tokenToTokenSwap(uint256 _tokenSold, uint256 _minTokenBought, address _tokenAddress) public {
        address exchangeAddress = IFactory(factoryAddress).getExchange(_tokenAddress);
        require(exchangeAddress != address(this) && exchangeAddress != address(0), "invalid exchange address");

        IERC20(_tokenAddress).transferFrom(msg.sender, address(this), _tokenSold);

        uint256 ethBought = getEthAmount(_tokenSold);

        Exchange exchange = Exchange(exchangeAddress);
        exchange.ethToToken{value: ethBought}(_minTokenBought, msg.sender);
    }
}
