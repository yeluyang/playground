// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IERC20} from "./IERC20.sol";

contract Faucet {
    error ErrAlreadyRequested(address sender);

    uint256 public constant amountAllowed = 100;
    address public immutable tokenContract;
    mapping(address => bool) public requestedAddress;

    event SendToken(address indexed receiver, uint256 amount);

    constructor(address _tokenContract) {
        tokenContract = _tokenContract;
    }

    modifier onlyNewComer() {
        if (requestedAddress[msg.sender] == false) {
            revert ErrAlreadyRequested(msg.sender);
        }
        _;
    }

    function requestTokens() external onlyNewComer {
        IERC20 token = IERC20(tokenContract);
        token.transfer(msg.sender, amountAllowed);
        requestedAddress[msg.sender] = true;

        emit SendToken(msg.sender, amountAllowed);
    }
}
