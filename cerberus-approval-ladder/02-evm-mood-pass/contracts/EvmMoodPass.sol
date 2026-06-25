// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @notice Minimal EVM-only NFT-like membership pass.
/// @dev This is intentionally not a Gear/Vara Sails program. It exists to
/// demonstrate why "on-chain somewhere" is not enough for VAN Application review.
contract EvmMoodPass {
    string public constant name = "EvmMoodPass";
    string public constant symbol = "MOOD";

    address public owner;
    uint256 public nextTokenId = 1;

    mapping(uint256 => address) public ownerOf;
    mapping(address => uint256) public passOf;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event PassMinted(address indexed account, uint256 indexed tokenId);

    error AlreadyMinted();
    error ZeroAddress();

    constructor() {
        owner = msg.sender;
    }

    function mint() external returns (uint256 tokenId) {
        if (msg.sender == address(0)) revert ZeroAddress();
        if (passOf[msg.sender] != 0) revert AlreadyMinted();

        tokenId = nextTokenId++;
        ownerOf[tokenId] = msg.sender;
        passOf[msg.sender] = tokenId;

        emit Transfer(address(0), msg.sender, tokenId);
        emit PassMinted(msg.sender, tokenId);
    }

    function hasPass(address account) external view returns (bool) {
        return passOf[account] != 0;
    }
}
