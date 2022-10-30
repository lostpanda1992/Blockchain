// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.8.0; //控制智能合约编译器的版本

contract Adoption {
    address[16] public adopters; // 保存领养者的地址,是包括以太坊地址的数组

    // 领养宠物
    function adopt(uint petId) public returns (uint) {
        require(petId >= 0 && petId <= 15); // 确保id在数组长度内
        adopters[petId] = msg.sender; // 保存调用这地址
        return petId;
    }

    // 返回领养者
    function getAdopters() public view returns (address[16] memory) {
        return adopters;
    }
}