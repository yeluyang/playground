// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IERC165} from "forge-std/interfaces/IERC165.sol";
import {IERC721, IERC721TokenReceiver, IERC721Metadata, IERC721Enumerable} from "forge-std/interfaces/IERC721.sol";

interface MyIERC721 is IERC165, IERC721, IERC721TokenReceiver, IERC721Metadata, IERC721Enumerable {}

contract ERC721 is MyIERC721 {
    address public owner;

    mapping(address => uint256) public balanceOf;
    mapping(uint256 => address) public ownerOf;
    mapping(uint256 => address) public allowance;

    error ErrAuthDenied(uint256 tokenID, address requstor);
    error ErrNotTokenOwner(uint256 tokenID, address owner, address requstor);

    constructor() {
        owner = msg.sender;
    }

    function checkAuth(uint256 id) internal view {
        if (ownerOf[id] != msg.sender && allowance[id] != msg.sender) {
            revert ErrAuthDenied(id, msg.sender);
        }
    }

    function checkOwnership(uint256 id, address from) internal view {
        if (ownerOf[id] != from) {
            revert ErrNotTokenOwner(id, ownerOf[id], from);
        }
    }

    function supportsInterface(bytes4 interfaceID) external view returns (bool) {
        return interfaceID == type(IERC721).interfaceId || interfaceID == type(IERC721Enumerable).interfaceId
            || interfaceID == type(IERC721Metadata).interfaceId || interfaceID == type(IERC721TokenReceiver).interfaceId
            || interfaceID == type(IERC165).interfaceId;
    }

    function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes calldata data) external payable {
        checkAuth(_tokenId);
        checkOwnership(_tokenId, _from);
        ownerOf[_tokenId] = _to;
        balanceOf[_from] -= 1;
        balanceOf[_to] += 1;
    }
}
