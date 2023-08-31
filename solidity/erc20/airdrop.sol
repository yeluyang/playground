// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./IERC20.sol";

contract AirDrop {
    error ErrLengthNotMatch(uint256 addrLeng, uint256 amountLen);
    error ErrSenderBalanceNotEnough(uint256 value, uint256 amount);

    function getSum(uint256[] calldata _arr) public pure returns (uint256 sum) {
        for (uint256 i = 0; i < _arr.length; i++) {
            sum += _arr[i];
        }
    }

    function multiTransferToken(address _token, address[] calldata _addresses, uint256[] calldata _amounts) external {
        if (_addresses.length != _amounts.length) {
            revert ErrLengthNotMatch(_addresses.length, _amounts.length);
        }
        IERC20 token = IERC20(_token);
        for (uint256 i = 0; i < _addresses.length; i++) {
            token.transfer(_addresses[i], _amounts[i]);
        }
    }

    function multiTransferETH(address payable[] calldata _addresses, uint256[] calldata _amounts) public payable {
        if (_addresses.length != _amounts.length) {
            revert ErrLengthNotMatch(_addresses.length, _amounts.length);
        }
        uint256 sum = getSum(_amounts);
        if (msg.value < sum) {
            revert ErrSenderBalanceNotEnough(msg.value, sum);
        }
        for (uint256 i = 0; i < _addresses.length; i++) {
            _addresses[i].transfer(_amounts[i]);
        }
    }
}
