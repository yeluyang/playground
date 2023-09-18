// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.0;

interface IFactory {
    function getExchange(address _tokenAddress) external view returns (address);
}
