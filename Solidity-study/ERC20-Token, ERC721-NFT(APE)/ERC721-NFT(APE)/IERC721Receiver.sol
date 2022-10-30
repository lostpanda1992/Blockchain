// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// ERC721接收者接口：合约必须实现这个接口来通过安全转账接受ERC721
// 如果一个合约没有实现ERC721的相关函数，转入的NFT就进了黑洞，永远转不出来了
// 为了防止误转账，ERC721实现了safeTransferFrom()安全转账函数
// 目标合约必须实现了IERC721Receiver接口才能接收ERC721代币，不然会revert
interface IERC721Recevier {
    function onERC721Received(
        address operator,
        address from,
        uint tokenId,
        bytes calldata data
    ) external returns (bytes4);
}