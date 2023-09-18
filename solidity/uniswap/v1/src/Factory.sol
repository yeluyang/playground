// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import {IFactory} from "./IFactory.sol";
import {Exchange} from "./Exchange.sol";

contract Factory is IFactory {
    mapping(address => address) public tokenToExchange;

    function createExchange(address _tokenAddress) public returns (address) {
        require(_tokenAddress != address(0), "invalid token address");
        require(tokenToExchange[_tokenAddress] == address(0), "exchange already exists");
        Exchange exchange = new Exchange(_tokenAddress);
        tokenToExchange[_tokenAddress] = address(exchange);
        return address(exchange);
    }

    function getExchange(address _tokenAddress) external view returns (address) {
        return tokenToExchange[_tokenAddress];
    }
}
