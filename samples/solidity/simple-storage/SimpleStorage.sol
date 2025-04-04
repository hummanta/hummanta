// SPDX-License-Identifier: GPL-3.0
// src: https://docs.soliditylang.org/en/latest/introduction-to-smart-contracts.html#storage-example
pragma solidity >=0.4.16 <0.9.0;

contract SimpleStorage {
    uint storedData;

    function set(uint x) public {
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}
