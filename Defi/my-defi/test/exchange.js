
const { token } = require("@metaplex-foundation/js");
const {expect} = require("chai");

describe("add liquidity", function(){
    let token, exchange
    beforeEach(async() =>{
        totalsupply = ethers.utils.parseEther('21000000');
        // 部署token.sol
        const Token = await ethers.getContractFactory("Token");
        token = await Token.deploy("Bitcoin","BTC",totalsupply);

        // 部署exchange.sol
        const Exchange = await ethers.getContractFactory("Exchange");
        exchange = await Exchange.deploy(token.address);

        tokenamount = ethers.utils.parseEther('200');
        ethnamount = ethers.utils.parseEther('100');

        //向exchange转200
        await token.approve(exchange.address, tokenamount);
        // 加入流动性 200个token 100个以太
        await exchange.addLiquidity(tokenamount, {value:ethnamount});

        expect(await exchange.getReserve()).to.equal(tokenamount);
    })

    describe("get token amount", function(){
        it("返回正确的token数量", async function(){
            const tokensOut= await exchange.getTokenAmount(ethers.utils.parseEther('1'))
            console.log(ethers.utils.formatEther(tokensOut));
        });

        it("返回正确的eth数量", async function(){
            const ethOut= await exchange.getEthAmount(ethers.utils.parseEther('1'))
            console.log(ethers.utils.formatEther(ethOut));
        });
    });
});