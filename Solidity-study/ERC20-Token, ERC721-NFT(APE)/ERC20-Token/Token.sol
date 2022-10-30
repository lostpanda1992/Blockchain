// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "./IERC20.sol";

///实现一个ERC20代币，简单实现IERC20规定的函数
contract Token is IERC20 {
    //账户余额
    mapping(address => uint256) public override balanceOf;
    // `owner`账户授权给`spender`账户的额度，默认为0
    mapping(address => mapping(address => uint256)) public override allowance;
    //货币总供给量
    uint256 public override totalSupply; 

    //名称
    string public name;
    //代号
    string public symbol;

    //小数位数
    uint8 public decimals = 18;

    //初始化代币名称、代号
    constructor(string memory name_, string memory symbol_) {
        name = name_;
        symbol = symbol;
    }

    // 实现IERC20中的transfer函数，代币转账逻辑。调用方扣除amount数量代币，接收方增加相应代币
    function transfer(address recipient, uint amount) external override returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[recipient] += amount;
        emit Transfer(msg.sender, recipient, amount);
        return true;
    }

    // 实现IERC20中的approve函数，代币授权逻辑。被授权方spender可以支配授权方的amount数量的代币
    // 仅授权 未发生货币转移
    function approve(address spender, uint amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    // 实现IERC20中的transferFrom函数，授权转账逻辑。被授权方将授权方sender的amount数量的代币转账给接收方recipient
    // 被授权方为approve函数中的spender，当前函数中的msg.sender
    // 该函数发生在approve函数之后
    // 当前函数调用者为中间人，将sender的货币转移给recipient
    function transferFrom(address sender, address recipient, uint amount) external override returns (bool) {
        allowance[sender][msg.sender] -= amount; //将allowance中记录清空，防止双重攻击
        balanceOf[sender] -= amount; // 扣除sender货币
        balanceOf[recipient] += amount;
        emit Transfer(sender, recipient, amount);
        return true;
    }

    // 铸造代币函数，不在IERC20标准中.这里任何人都能铸造任意数量的代币，实际需要权限管理
    function mint(uint amount) external {
        // require(msg.sender == owner)
        balanceOf[msg.sender] += amount;
        totalSupply += amount;
        emit Transfer(address(0), msg.sender, amount); // 事件记录：发出方为空，接收方为msg.sender,数量为amount
    }

    // burn()函数：销毁代币函数，不在IERC20标准中
    function burn(uint amount) external {
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        emit Transfer(msg.sender, address(0), amount); // msg.sender向空地址转移了amount的货币 实际上是销毁了
    }
}