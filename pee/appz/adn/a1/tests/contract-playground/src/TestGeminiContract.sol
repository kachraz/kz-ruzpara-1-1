// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title TestGeminiContract
 * @dev A test contract with various security issues for Gemini AI analysis
 */
contract TestGeminiContract {
    address public owner;
    mapping(address => uint256) public balances;
    uint256 public totalSupply;
    bool private locked;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Not the owner");
        _;
    }
    
    modifier noReentrancy() {
        require(!locked, "Reentrant call");
        locked = true;
        _;
        locked = false;
    }
    
    constructor() {
        owner = msg.sender;
        totalSupply = 1000000 * 10**18;
        balances[msg.sender] = totalSupply;
    }
    
    // VULNERABILITY: Missing access control
    function mint(address to, uint256 amount) external {
        balances[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }
    
    // VULNERABILITY: Arbitrary transfer from
    function transferFrom(address from, address to, uint256 amount) external {
        require(balances[from] >= amount, "Insufficient balance");
        balances[from] -= amount;
        balances[to] += amount;
        emit Transfer(from, to, amount);
    }
    
    // VULNERABILITY: Reentrancy risk
    function withdraw(uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        // External call before state change
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        
        balances[msg.sender] -= amount; // State change after external call
    }
    
    // VULNERABILITY: Integer overflow (pre-0.8.0 style)
    function unsafeAdd(uint256 a, uint256 b) external pure returns (uint256) {
        return a + b; // Could overflow in older versions
    }
    
    // VULNERABILITY: Weak randomness
    function randomNumber() external view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(block.timestamp, block.difficulty, msg.sender)));
    }
    
    // VULNERABILITY: Unprotected selfdestruct
    function destroy() external {
        selfdestruct(payable(msg.sender));
    }
    
    // VULNERABILITY: tx.origin usage
    function authWithTxOrigin() external {
        require(tx.origin == owner, "Not authorized");
        // Dangerous logic here
    }
    
    // GOOD: Proper access control
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "New owner is the zero address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }
    
    // GOOD: Protected function with reentrancy guard
    function safeWithdraw(uint256 amount) external noReentrancy {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        balances[msg.sender] -= amount; // State change first
        
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
    }
    
    // VULNERABILITY: Missing zero address check
    function setOwner(address newOwner) external onlyOwner {
        owner = newOwner; // Should check for zero address
    }
    
    // VULNERABILITY: Unchecked external call
    function unsafeExternalCall(address target, bytes calldata data) external onlyOwner {
        target.call(data); // Return value not checked
    }
    
    // GOOD: Proper external call handling
    function safeExternalCall(address target, bytes calldata data) external onlyOwner returns (bool) {
        (bool success, ) = target.call(data);
        return success;
    }
    
    receive() external payable {
        balances[msg.sender] += msg.value;
    }
    
    fallback() external payable {
        revert("Function not found");
    }
}