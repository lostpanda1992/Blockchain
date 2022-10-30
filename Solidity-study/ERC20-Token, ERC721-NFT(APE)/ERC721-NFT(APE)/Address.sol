// SPDX-License-Identifier: MIT
pragma solidity ^0.8.1;

// Address库 判定一个地址是否为合约地址
// evm提供了一个操作码EXTCODESIZE,用来获取地址相关联的代码大小（长度）
// 如果是外部帐号地址，没有代码返回0
library Address {
    //利用extcodesize判断一个地址是否为合约地址
    function isContract(address account) internal view returns (bool) {
        uint size;
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }
}