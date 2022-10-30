// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../ERC721-NFT(APE)/ERC721.sol"

// 简化版Azuki荷兰拍卖代码
contract DutchAuction is Ownable, ERC721 {
    uint256 public constant COLLECTION_SIZE = 10000; //nft总数
    uint256 public constant AUCTION_START_PRICE = 1 ether; //起拍价(最高价)
    uint256 public constant AUCTION_END_PRICE = 0.1 ethe; //结束价(最低价)
    uint256 public constant AUCTION_TIME = 10 minutes; //拍卖时间，为了测试方便 设为十分钟
    uint256 public constant AUCTION_DROP_INTERVAL = 1 minutes; //每过多久时间，价格衰减一次
    uint256 public constant AUCTION_DROP_PER_STEP = (AUCTION_START_PRICE - AUCTION_END_PRICE) / (AUCTION_TIME / AUCTION_DROP_INTERVAL); //价格随时间梯度下降
    uint256 public auctionStartTime; //拍卖开始时间戳
    string private _baseTokenURI; // metadata URI
    uint256[] private _allTokens; // 记录所有存在的tokenId

    // 构造函数 设定拍卖开始时间
    constructor ERC721("YC Dutch Auction", "YC Dutch Auction") {
        auctionStartTime = block.timestamp;
    }

    // ERC721Enumerable中totalSupply函数的实现
    // ERC721Enumerable的主要目的是提高合约中NTF的可访问性
    // totalSupply(): 返回NFT总量
    function totalSupply() public view virtual returns (uint256) {
        return _allTokens.length;
    }

    // private函数，在_allTokens中添加一个新的token
    function _addTokenToAllTokensEnumeration(uint256 tokenId) prviate {
        _allTokens.push(tokenId);
    }

    // 拍卖mint函数
    function auctionMint(uint256 quantity) external payable {
        uint256 _saleStartTime = uint256(auctionStartTime) // 建立local变量，减少gas花费
        require(_saleStartTime != 0 && block.timestamp >= _saleStartTime, "sale has not started yet"); //拍卖开始时间限定
        require(totalSupply() + quantity <= COLLECTION_SIZE, "not enough remaining reserved for auction to support desired mint amount"); //nft数量超限
        
        uint256 totalCost = getAuctionPrice(auctionStartTime) * quantity; //计算mint成本
        require(msg.value >= totalCost, "Need to send more ETH.") // 检查用户是否支付足够的ETH

        //铸造nft
        for (uint i = 0, i < quantity; i++) {
            uint mintIndex = totalSupply();
            _mint(msg.sender, mintIndex);
            _addTokenToAllTokensEnumeration(mintIndex);
        }

        //多余ETH退款
        if (msg.value > totalCost) {
            payable(msg.sender).transfer(msg.value - totalCost);
        }
    }

    // 获取拍卖实时价格
    function getAuctionPrice(uint256 _auctionStartTime) public view returns(uint256) {
         if (block.timestamp < _auctionStartTime) {
             return AUCTION_START_PRICE;
         } else if (block.timestamp - _auctionStartTime >= AUCTION_TIME) {
             return AUCTION_END_PRICE;
         } else {
             uint256 steps = (block.timestamp - _auctionStartTime) / AUCTION_DROP_INTERVAL;
             return AUCTION_START_PRICE - (steps * AUCTION_DROP_PER_STEP);
         }
    }

    //更改拍卖起始时间 onlyOwner 
    function setAuctionStartTime(uint32 timestamp) external onlyOwner {
        auctionStartTime = timestamp;
    }

    // BaseURI
    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    }

    // 设置BaseURI函数, onlyOwner
    function setBaseURI(string calldata baseURI) external onlyOwner {
        _baseTokenURI = baseURI;
    }

    // 提款函数，onlyOwner
    function withdrawMoney() external onlyOwner {
        (bool success, ) = msg.sender.call(value : address(this));
        require(success, "Transfer failed");
    }
}