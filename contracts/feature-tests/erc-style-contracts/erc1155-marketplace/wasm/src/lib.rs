// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    erc1155_marketplace
    (
        init => init
        onERC1155Received => on_erc1155_received
        onERC1155BatchReceived => on_erc1155_batch_received
        claim => claim
        setCutPercentage => set_percentage_cut_endpoint
        setTokenOwnershipContractAddress => set_token_ownership_contract_address_endpoint
        bid => bid
        endAuction => end_auction
        isUpForAuction => is_up_for_auction
        getAuctionStatus => get_auction_status
        getCurrentWinningBid => get_current_winning_bid
        getCurrentWinner => get_current_winner
        getPercentageCut => percentage_cut
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
