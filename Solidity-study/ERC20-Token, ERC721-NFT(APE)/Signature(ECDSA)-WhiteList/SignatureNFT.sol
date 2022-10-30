// SPDX-License-Identifier: MIT
// By 0xAA
pragma solidity ^0.8.4;

import "../ERC721-NFT(APE)/ERC721.sol";

// ECDSA库
library ECDSA {
    /**
     * @dev 通过ECDSA，验证签名地址是否正确，如果正确则返回true
     * _msgHash为消息的hash
     * _signature为签名
     * _signer为签名地址
     */
    function verify(bytes32 _msgHash, bytes memory _signature, address _signer) public pure returns (bool) {
        return recoverSigner(_msgHash, _signature) = _signer;
    }

    // 从_msgHash和签名_signature中恢复signer地址
    function recoverSigner(bytes32 _msgHash, bytes memory _signature) public pure returns (address) {
        // 检查签名长度，65是标准r,s,v签名的长度
        require(_signature.length == 65, "invalid signature length");
        bytes32 r;
        bytes32 s;
        uint8 v;
        
        // 目前只能用assembly (内联汇编)来从签名中获得r,s,v的值
        assembly {
            /*
            前32 bytes存储签名的长度 (动态数组存储规则)
            add(sig, 32) = sig的指针 + 32
            等效为略过signature的前32 bytes
            mload(p) 载入从内存地址p起始的接下来32 bytes数据
            */
            // 读取长度数据后的32 bytes
            r := mload(add(_signature, 0x20))
            // 读取之后的32 bytes
            s := mload(add(_signature, 0x40))
            // 读取最后一个byte
            v := byte(0, mload(add(_signature, 0x60)))
        }
        // 使用ecrecover(全局函数)：利用 msgHash 和 r,s,v 恢复 signer 地址
        return ecrecover(_msgHash, v, r, s);
    }

    /**
     * @dev 返回 以太坊签名消息
     * `hash`：消息哈希 
     * 遵从以太坊签名标准：https://eth.wiki/json-rpc/API#eth_sign[`eth_sign`]
     * 以及`EIP191`:https://eips.ethereum.org/EIPS/eip-191`
     * 添加"\x19Ethereum Signed Message:\n32"字段，防止签名的是可执行交易。
     */
    function toEthSignedMessageHash(bytes32 hash) public pure returns (bytes32) {
        // 32 is the length in bytes of hash,
        // enforced by the type signature above
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }
}

contract SignatureNFT is ERC721 {
    address immutable public signer; //签名地址
    mapping(address => bool) public mintedAddress; //记录已经minted的地址

    // 构造函数，初始化NFT合集的名称、代号、签名地址
    constructor(string memory _name, string memory _symbol, address _signer) RC721(_name, _symbol) {
        signer = _signer;
    }

    // 利用ECDSA验证签名并mint
    function mint(address _account, uint256 _tokenId, bytes memory _signature) external {
        bytes32 _msgHash = getMessageHash(_account, _tokenId); //将_account和_tokenId打包消息
        bytes32 _ethSignedMessageHash = ECDSA.toEthSignedMessageHash(_msgHash); // 计算以太坊签名消息
        require(verify(_ethSignedMessageHash, _signature), "Invalid signature"); // ECDSA检验通过
        require(!mintedAddress[_account], "Already minted!"); //地址没有mint过

        mintedAddress[_account] = true; //记录mint过的地址
        _mint(_accunt, _tokenId); // mint
    }

    /*
     * 将mint地址（address类型）和tokenId（uint256类型）拼成消息msgHash
     * _account: 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4
     * _tokenId: 0
     * 对应的消息msgHash: 0x1bf2c0ce4546651a1a2feb457b39d891a6b83931cc2454434f39961345ac378c
     */
    function getMessageHash(address _account, uint256 _tokenId) public pure returns(bytes32){
        return keccak256(abi.encodePacked(_account, _tokenId));
    }

    // ECDSA验证，调用ECDSA库的verify()函数
    function verify(bytes32 _msgHash, bytes memory _signature) public view returns (bool){
        return ECDSA.verify(_msgHash, _signature, signer);
    }
}

/*签名验证
如何签名和验证
#签名
1.创建要签名的消息
2.计算消息的hash
3.给hash过得消息签名（链下，注意私钥保密）

#验证
1.从原始消息重新创建hash
2.从签名地址和哈希恢复签名
3.将恢复的签名与原始的签名进行比较
*/

contract VerifySignature {
    /* 1. Unlock MetaMask account
    ethereum.enable()
    */
    // 第一步：解锁metamask账户 

    /* 2. Get message hash to sign
    getMessageHash(
        0x14723A09ACff6D2A60DcdF7aA4AFf308FDDC160C,
        123,
        "coffee and donuts",
        1
    )
    hash = "0xcf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cd"
    */
    // 第二步 获得消息hash
    function getMessageHash(address _addr, uint256 _tokenId) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(_addr, _tokenId));
    }

    /* 3. Sign message hash
    # using browser
    account = "copy paste account of signer here"
    ethereum.request({ method: "personal_sign", params: [account, hash]}).then(console.log)
    # using web3
    web3.personal.sign(hash, web3.eth.defaultAccount, console.log)
    Signature will be different for different accounts
    0x993dab3dd91f5c6dc28e17439be475478f5635c92a56e17e82349d3fb2f166196f466c0b4e0c146f285204f0dcb13e5ae67bc33f4b888ec32dfe0a063e8f3f781b
    */
    // 签名hash消息 1.利用浏览器 2.利用web3.js
    function getEthSignedMessageHash(bytes32 _messageHash) public pure returns (bytes32) {
        /*
        Signature is produced by signing a keccak256 hash with the following format:
        "\x19Ethereum Signed Message\n" + len(msg) + msg
        */
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", _messageHash));
    }

     /* 4. Verify signature
    signer = 0xB273216C05A8c0D4F0a4Dd0d7Bae1D2EfFE636dd
    to = 0x14723A09ACff6D2A60DcdF7aA4AFf308FDDC160C
    amount = 123
    message = "coffee and donuts"
    nonce = 1
    signature =
        0x993dab3dd91f5c6dc28e17439be475478f5635c92a56e17e82349d3fb2f166196f466c0b4e0c146f285204f0dcb13e5ae67bc33f4b888ec32dfe0a063e8f3f781b
    */
    // 验证签名
    function verify(address _signer, address _addr, uint _tokenId, bytes memory signature) public pure returns (bool) {
        bytes32 messageHash = getMessageHash(_addr, _tokenId);
        bytes32 ethSignedMessageHash = getEthSignedMessageHash(messageHash);
        return recoverSigner(ethSignedMessageHash, signature) == _signer;
    }

    function recoverSigner(bytes32 _ethSignedMessageHash, bytes memory _signature) public pure returns (address) {
        (bytes32 r, bytes32 s, uint8 v) = splitSignature(_signature);
        return ecrecover(_ethSignedMessageHash, v, r, s);
    }

    function splitSignature(bytes memory sig) public pure returns (bytes32 r, bytes32 s, uint8 v) {
        // 检查签名长度，65是标准r,s,v签名的长度
        require(sig.length == 65, "invalid signature length");
        assembly {
            /*
            First 32 bytes stores the length of the signature
            add(sig, 32) = pointer of sig + 32
            effectively, skips first 32 bytes of signature
            mload(p) loads next 32 bytes starting at the memory address p into memory
            */
            // first 32 bytes, after the length prefix
            r := mload(add(sig, 0x20))
            // second 32 bytes
            s := mload(add(sig, 0x40))
            // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(sig, 0x60)))
        }
        // implicitly return (r, s, v)
    }
}