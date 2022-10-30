// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "../libraries/Math.sol";
import "../libraries/ERC20.sol";

interface IERC20 {
    function balanceOf(address) external returns (uint256);
    function transfer(address to, uint256 amount) external;
}

error InsufficientLiquidityMinted();
error InsufficientLiquidityBurned();
error TransferFailed();

contract UniswapV2Pair is ERC20, Math {
    uint256 constant MINIMUM_LIQUIDITY = 1000;
    
    address public token0; //两个币的地址
    address public token1;

    uint112 private reserve0; //池子内token余额
    uint112 private reserve1;

    constructor(address token0_, address token1_) ERC20("UniswapV2Pair", "UNIV2", 18) {
        token0 = token0_;
        token1 = token1_;
    }

    // 添加流动性 假设该池子内已经有对应代币了
    function mint() public {
        (uint112 _reserve0, uint112 _reserve1, ) = getReserves(); //获取token0和token1的数量

        uint256 balance0 = IERC20(token0).balanceOf(address(this)); //添加流动性之后的token0数量
        uint256 balance1 = IERC20(token1).balanceOf(address(this)); //添加流动性之后的token1数量

        uint256 amount0 = balance0 - _reverse0; // 用户往合约中添加了多少流动性 token0
        uint256 amount1 = balance1 - _reverse1; // 用户往合约中添加了多少流动性 token1

        uint256 liquidity; //该给用户多少流动性()

        //V2只有token liquidity = sqrt(amount0 * amount1) 几何平均值
        if (totalSupply == 0) {
            //当池子中没有流动性的时候 第一次
            liquidity = Math.sqrt(amount0 * amount1) - MINIMUM_LIQUIDITY;
            _mint(address(0), MINIMUM_LIQUIDITY); //给黑洞（address(0)添加一些流动性，保证池子不会被抽空，即永远有一部分流动性存在）
        } else { //第二次添加流动性
            //防止用户胡乱添加流动性（不按比例） 将流动性增加值较小的作为流动性增加值
            liquidity = Math.min((totalSupply * amount0) / _reverse0, 
            (totalSupply * amount1) / _reverse1);
        }
        //如果一开始没有流动性 这里会revert
        if (liquidity <= 0) revert InsufficientLiquidityMinted();

        _mint(msg.sender, liquidity); //lq代币发给用户
        _update(balance0, balance1); //更新balance

        emit Mint(msg.sender, amount0, amount1);
    }

    function getReserves() public view returns(uint112, uint112, uint32) {
        return (reverse0, reverse1, 0);
    }

    function _update(uint256 balance0, uint256 balance1) private {
        reserve0 = uint112(balance0); //最终的余额更新到reserve中
        reverse1 = uint112(balance1);

        emit Sync(reserve0, reserve1);
    }

}