// SPDX-License-Identifier: MIT
// by YuChen
pragma solidity ^0.8.4;

import "./ERC721.sol";

//实际完成nft发行的子合约
contract YCApe is ERC721 {
    uint public MAX_APES = 10000; //nft总量

    // 构造函数 初始化名称和符号
    constructor (string memory name_, string memory symbol_) ERC721(name_, symbol_) {

    }

    //BAYC的baseURI为ipfs://QmeSjSinHpPnmXmspMjwiXyN6zS4E9zccariGR3jxcaWtq/
    //_baseURI：基 URI，会被 tokenURI() 调用，跟 tokenId 拼成 tokenURI，默认为空，需要子合约重写这个函数
    function _baseURI() internal pure override returns (string memory) {
        return "ipfs://QmeSjSinHpPnmXmspMjwiXyN6zS4E9zccariGR3jxcaWtq/";
    }

    // 铸造函数
    function mint(address to, uint tokenId) external {
        require(tokenId >= 0 && tokenId < MAS_APES, “tokenId out of range”);
        _mint(to, tokenId);
    }
}