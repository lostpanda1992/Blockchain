// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// IERC721Metadata是ERC721的拓展接口
// 实现了3个查询metadata元数据的常用函数
interface IERC721Metadata is IERC721 {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function tokenURI(uint256 tokenId) external view returns (string memory);
}