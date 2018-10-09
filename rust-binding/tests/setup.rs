//
// Copyright (c) 2011-2018, UDI Contributors
// All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
#![deny(warnings)]

extern crate native_file_tests;

use std::path::PathBuf;

#[test]
fn setup() {
    let nft_dir = std::env::var("NFT_DIR").unwrap();

    let nft_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join(nft_dir);

    native_file_tests::create_test_metadata(&nft_path,
                                            &std::env::consts::OS);
}
