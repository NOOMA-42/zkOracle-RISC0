// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiCallbackReceiver} from "bonsai/BonsaiCallbackReceiver.sol";

/// @title A starter application using Bonsai through the on-chain relay.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function to a RISC Zero guest running on Bonsai.
contract PriceOracle is BonsaiCallbackReceiver {
    uint256 public price;

    /// @notice Image ID of the only zkVM binary to accept callbacks from.
    bytes32 public immutable imageId;

    /// @notice Gas limit set on the callback from Bonsai.
    /// @dev Should be set to the maximum amount of gas your callback might reasonably consume.
    uint64 private constant BONSAI_CALLBACK_GAS_LIMIT = 100000;

    /// @notice Initialize the contract, binding it to a specified Bonsai relay and RISC Zero guest image.
    constructor(
        IBonsaiRelay bonsaiRelay,
        bytes32 _imageId
    ) BonsaiCallbackReceiver(bonsaiRelay) {
        imageId = _imageId;
    }

    event CalculateFibonacciCallback(uint256 indexed n, uint256 result);

    /// @notice Callback function logic for processing verified journals from Bonsai.
    function storeResult(uint256 _price) external onlyBonsaiCallback(imageId) {
        price = _price;
    }

    /// @notice Sends a request to Bonsai to have have the nth Fibonacci number calculated.
    /// @dev This function sends the request to Bonsai through the on-chain relay.
    ///      The request will trigger Bonsai to run the specified RISC Zero guest program with
    ///      the given input and asynchronously return the verified results via the callback below.
    function requestPrice() external {
        bonsaiRelay.requestCallback(
            imageId,
            "",
            address(this),
            this.storeResult.selector,
            BONSAI_CALLBACK_GAS_LIMIT
        );
    }
}
