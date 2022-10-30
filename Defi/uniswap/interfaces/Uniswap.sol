// SPDX-License-Identifier: MIT
pragma solidity ^0.7;

interface IUniswapV2Pair {
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata paht,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);
}