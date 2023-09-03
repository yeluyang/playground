pragma solidity ^0.8.0;

import {Test, console2} from "forge-std/Test.sol";
import {ERC20} from "../src/ERC20.sol";
import {AirDrop} from "../src/airdrop.sol";

contract AirDropTest is Test {
    ERC20 token;
    AirDrop airDrop;

    function setUp() public {
        token = new ERC20("wtf", "wtf");
        airDrop = new AirDrop(address(token));
    }
}
