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

    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let nft_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join(nft_dir);

    native_file_tests::setup(&nft_path,
                             &out_path,
                             &std::env::consts::OS);

    assert!(out_path.join("native_file_tests.rs").exists());
}
