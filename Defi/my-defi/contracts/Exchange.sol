// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

//引入接口合约 将用户的币转入池子
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

interface IFactory{
    function getExchange(address _tokenAddress) external view returns(address);
}

interface IExchange{
    function ethToTokenTransfer(uint256 _minTokens, address recipent) public payable;
}

// 使以太坊和token交互
contract Exchange is ERC20{
    address public tokenAddress; //该token地址public
    address public factoryAddress; //工厂合约地址 上下联通数据

    constructor(address _tokenAddress, _factoryAddress) ERC20("YCswap-V1", "YC_V1"){
        require(_tokenAddress != address(0), "invalid address"); // 确认地址合法
        tokenAddress = _tokenAddress; //初始化时将token地址赋值给池子地址
        factoryAddress = _factoryAddress; //构造函数时传入工厂合约地址
    }

    // 可能存在价格操控 添加流动性 即添加代币 要确保比例一致
    function addLiquidity(uint256 _amount) public payable returns(uint256){
        if (getReserve() == 0) { //当前池子为空时
            IERC20 token = IERC20(tokenAddress); // 转换为ierc20代币
            token.transferFrom(msg.sender, address(this), _amount); //用户的钱转入池子中

            // 创建池子 提供流动性凭证 第一次totalamount为零
            //公式：amountminted = totalamount * ethreserve / ethdeposit 
            uint256 liquidity = address(this).balance;
            _mint(msg.sender, amount); // 第一次直接将以太币对应的数量的代币发给创建者
            return liquidity; //返回数量
        } else {
            // 限制池子添加比例 
            uint256 ethReserve = address(this).balance - msg.value; // 当前池子以太坊余额 - 用户以太坊余额
            uint256 tokenReserve = getReserve();

            // 确保池子比例 
            uint256 tokenAmount = (msg.value * tokenReserve) / ethReserve;
            // 向池子中添加的币比用户给的少才行
            require(tokenAmount <= _amount, "invalid address"); // 确认地址合法

            IERC20 token = IERC20(tokenAddress); // 转换为ierc20代币
            token.transferFrom(msg.s.sender, address(this),_amount); //用户的币转移至合约

            //公式：amountminted = totalamount * ethdeposit /  ethreserve
            uint256 liquidity = (totalSupply() *msg.value) / ethReserve;
            _mint(msg.sender, liquidity); 
            return liquidity; //返回数量
        }
    }

    // 移除流动性 不用接受以太 所以无payable, 移除为两个 即以太和代币都有可能
    function removeLiquidity(uint256 _amount) public returns(uint256, uint256) {
        require(_amount > 0, "invaild");
        // 怎么知道该移除多少以太坊？(移除的数量除以池子的总量) 乘以总共的以太坊数量
        uint256 ethAmount = _amount * address(this).balance/ totalSupply();
        // token同理
        uint256 tokenAmount = _amount * getReserve()/ totalSupply();

        //移除流动性后要燃烧对应代币
        _burn(msg.sender, _amount);

        payable(msg.sender).transfer(ethAmount);
        // 不用transferFrom 因为是给用户发币
        IERC20(tokenAddress).transfer(msg.sender, tokenAmount); 
        return (ethAmount, tokenAmount);
    }

    function getReserve() public view returns (uint256) {
         return IERC20(tokenAddress).balanceOf(address(this)); //这个合约有多少代币
    }

    // 恒定乘积法的实现 核心公式 
    function getAmount(uint256 inputAmount, uint256 inputReserve, uint256 outputReserve) private pure returns(uint256){
        require(inputAmount > 0 && outputReserve > 0, "invaild amount");

        uint256 inputAmountWithFee = inputAmount * 99;
        uint256 numerator = inputAmountWithFee * outputReserve;
        uint256 denominator = inputAmountWithFee + (inputReserve * 100);

        return numerator / denominator;
    }

    //对外功能 查看一定的以太能获得多少代币
    function getTokenAmount(uint256 _ethSold) public view returns(uint256){
        require(_ethSold > 0 , "invaild");

        uint256 tokenReserve = getReserve();
        // 用户输入的以太坊量，合约中以太坊量，合约中代币数量
        return getAmount(_ethSold, address(this).balance, tokenReserve);
    }

    //对外功能 查看一定的代币能获得多少以太
    function getEthAmount(uint256 _tokenSold) public view returns(uint256){
        require(_tokenSold > 0 , "invaild");

        uint256 tokenReserve = getReserve();
        // 用户输入的以太坊量，合约中以太坊量，合约中代币数量
        return getAmount(_tokenSold, tokenReserve, address(this).balance);
    }

    // // 前端计算得到 包括滑点 用户用以太坊换多少代币
    // function ethToTokenSwap(uint256 _minTokens) public payable {
    //     // token总量
    //     uint256 tokenReserve = getReserve();
    //     // 卖出以太获得代币数量
    //     uint256 tokenBought = getAmount(msg.value, address(this).balance - msg.value, tokenReserve);

    //     //获得的代币大于最小获得量
    //     require(tokenBought >= _minTokens, "insufficient");
    //     //转移代币给用户
    //     IERC20(tokenAddress).transfer(msg.sender, tokenBought); 
    // }

    // 前端计算得到 包括滑点 用户用以太坊换多少代币
    // 变成一个包装器 调用ethToToken函数就行了
    function ethToTokenSwap(uint256 _minTokens) public payable {
        ethToToken(_minTokens, msg.sender);
    }

    // 前端计算得到 包括滑点 用户用以太坊换多少代币
    // 优化 原先是tokentotoken时 一池子调用二池子 二池子代币调给一池子 存在问题
    function ethToToken(uint256 _minTokens, address recipent) private {
        // token总量
        uint256 tokenReserve = getReserve();
        // 卖出以太获得代币数量
        uint256 tokenBought = getAmount(msg.value, address(this).balance - msg.value, tokenReserve);

        //获得的代币大于最小获得量
        require(tokenBought >= _minTokens, "insufficient");
        //转移代币给用户 修改 从msg.sender变成 recipent
        IERC20(tokenAddress).transfer(recipent, tokenBought); 
    }


    function ethToTokenTransfer(uint256 _minTokens, address recipent) public payable{
        ethToToken(_minTokens, recipent);
    }

    // 前端计算得到 包括滑点 用户输入代币换以太坊 最小获得以太坊数量
    function tokenToEthSwap(uint256 _tokenSold, uint256 _minEth) public {
        // token总量
        uint256 tokenReserve = getReserve();
        // 卖出以太获得代币数量
        uint256 ethBought = getAmount(_tokenSold, tokenReserve, address(this).balance);

        //获得的代币大于最小获得量
        require(ethBought >= _minEth, "insufficient");
        // 用户token放入合约池子中
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), _tokenSold); 

        //转移以太坊给用户
        payable(msg.sender).transfer(ethBought);
    }

    // 币币互换 与V2不同 目前是通过以太坊作为中介
    function tokenToToken(uint256 _tokenSold, uint256 _minTokenBought, address _tokenAddress) public {
        //首先找到对应池子的地址
        address exchangeAddress = IFactory(factoryAddress).getExchange(_tokenAddress);
        require(exchangeAddress != address(this) && exchangeAddress != address(0), 
            "invaild exchange address"); // 不能是本池子 并且对应的池子要存在

        // token to ether过程
        uint256 tokenReserve = getReserve();
        // 先买以太 输入为 当前卖出代币 当前该代币总量 本地址（以太坊中介池子中）以太坊数量
        uint256 ethBought = getAmount(_tokenSold, tokenReserve, address(this).balance);

        // 先将该种代币转移到合约中
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), _tokenSold);

        //用以太坊兑换目标代币 同时发送以太坊
        IExchange(exchangeAddress).ethToTokenTransfer{value : ethBought}(_minTokenBought);
    }
}