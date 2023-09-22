// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            1
// Async Callback:                       1
// Total number of exported functions:   3

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    transfer_role_features
    (
        init => init
        forwardPayments => forward_payments
    )
}

dharitri_sc_wasm_adapter::async_callback! { transfer_role_features }
