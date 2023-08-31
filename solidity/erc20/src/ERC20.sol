// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./IERC20.sol";

contract ERC20 is IERC20 {
    error ErrAuthDenied(address owner, address sender);
    error ErrBalanceNotEnough(address sender, uint256 balance, uint256 amount);
    error ErrAllowanceNotEnough(address from, address to, uint256 allowance, uint256 amount);
    error ErrSupplyNotEnough(uint256 supply, uint256 amount);

    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;

    uint8 public constant decimals = 18;
    address public immutable owner;
    string public name;
    string public symbol;

    constructor(string memory _name, string memory _symbol) {
        owner = msg.sender;
        name = _name;
        symbol = _symbol;
    }

    modifier onlyOwner() {
        if (msg.sender != owner) {
            revert ErrAuthDenied(owner, msg.sender);
        }
        _;
    }

    function checkBalance(address addr, uint256 amount) internal view {
        if (balanceOf[addr] < amount) {
            revert ErrBalanceNotEnough(addr, balanceOf[addr], amount);
        }
    }

    function checkSupply(uint256 amount) internal view {
        if (totalSupply < amount) {
            revert ErrSupplyNotEnough(totalSupply, amount);
        }
    }

    function checkAllowance(address from, address to, uint256 amount) internal view {
        if (allowance[from][to] < amount) {
            revert ErrAllowanceNotEnough(from, to, allowance[from][to], amount);
        }
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        checkBalance(msg.sender, amount);

        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        checkAllowance(from, msg.sender, amount);
        allowance[from][msg.sender] -= amount;

        checkBalance(from, amount);
        balanceOf[from] -= amount;
        balanceOf[to] += amount;

        emit Transfer(from, to, amount);
        return true;
    }

    function mint(uint256 amount) external onlyOwner {
        balanceOf[msg.sender] += amount;
        totalSupply += amount;
        emit Transfer(address(0), msg.sender, amount);
    }

    function burn(uint256 amount) external onlyOwner {
        checkBalance(msg.sender, amount);
        checkSupply(amount);
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        emit Transfer(msg.sender, address(0), amount);
    }
}
