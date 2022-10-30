// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "./Exchange.sol";
// 用作注册表，每个pair添加进来注册 
// 所有的合约都是通过工厂合约创建的
contract Factory {
    //类似于注册表
    mapping (address => address) public tokenToExchange;

    //创建池子 任何人都可以
    function createExchange(address _tokenAddress) public returns(address) {
        require(_tokenAddress != address(0), "invalid token address"); //合约地址不是一个无效地址
        // 确保没有基于address重复创建
        require(tokenToExchange[_tokenAddress] == address(0),"exchange already exists");

        // 创建一个新的池子合约 交互接口位置
        Exchange exchange= new Exchange(_tokenAddress, address(this));
        // 在注册表中注册该合约地址
        tokenToExchange[_tokenAddress] = address(exchange);
        return address(exchange);
    }

    //只读函数 不需要付手续费
    function getExchange(address _tokenAddress) public view returns(address) { 
        return tokenToExchange[_tokenAddress];
    }
}