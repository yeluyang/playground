// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IERC20} from "./IERC20.sol";

contract Faucet {
    error ErrAlreadyRequested(address sender);
    error ErrBalanceNotEnough(address owner, uint256 balance);

    uint256 public constant amountAllowed = 100;
    IERC20 public immutable token;
    mapping(address => bool) public requestedAddress;

    event SendToken(address indexed receiver, uint256 amount);

    constructor(address _tokenContract) {
        token = IERC20(_tokenContract);
    }

    modifier onlyNewComer() {
        if (requestedAddress[msg.sender]) {
            revert ErrAlreadyRequested(msg.sender);
        }
        _;
    }

    function requestTokens() external onlyNewComer {
        uint256 balance = token.balanceOf(address(this));
        if (balance < amountAllowed) {
            revert ErrBalanceNotEnough(address(this), balance);
        }

        token.transfer(msg.sender, amountAllowed);
        requestedAddress[msg.sender] = true;

        emit SendToken(msg.sender, amountAllowed);
    }
}
