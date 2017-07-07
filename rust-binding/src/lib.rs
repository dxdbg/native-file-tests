//
// Copyright (c) 2011-2017, UDI Contributors
// All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
#![deny(warnings)]

extern crate goblin;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

const SIMPLE_BINARY: &'static str =
    "simple-debug-noopt-dynamic.cd52194667df0781720ff834a56df134fef7fb51";

const WAITTHREAD_BINARY: &'static str =
    "waitthread-debug-noopt-dynamic.9bb7ace83d00e653b003e05b15ee7f9944c7f261";

pub fn setup(out_path: &PathBuf, zip_path: &PathBuf) {

    let dest_dir = out_path.join("native-file-tests");
    Command::new("unzip").arg("-j")
                         .arg(zip_path.to_str().unwrap())
                         .arg("-d")
                         .arg(dest_dir.to_str().unwrap())
                         .spawn()
                         .expect("Failed to start extract of native file tests zip")
                         .wait()
                         .expect("Failed to extract native file tests zip");

    let simple_path = dest_dir.join(SIMPLE_BINARY);
    let waitthread_path = dest_dir.join(WAITTHREAD_BINARY);

    let mod_file_path = out_path.join("native_file_tests.rs");
    let mut mod_file = File::create(&mod_file_path).unwrap();

    let symbols = build_symbols(&dest_dir);

    let sym_defs = symbols.iter()
                          .map(|e| format!("pub const {}: u64 = {};\n", e.0, e.1))
                          .fold("".to_owned(), |acc, elem| acc + &elem);

    let mod_file_content = format!("
pub const SIMPLE_EXEC_PATH: &'static str = \"{}\";
pub const WAITTHREAD_EXEC_PATH: &'static str = \"{}\";

{}
",
    simple_path.to_str().unwrap(),
    waitthread_path.to_str().unwrap(),
    sym_defs);

    mod_file.write_all(&mod_file_content.into_bytes())
            .expect("Failed to write native file tests module");
}

fn build_symbols(dest_dir: &PathBuf) -> HashMap<&str, String> {
    let mut symbols = HashMap::new();

    add_simple_binary_symbols(dest_dir, &mut symbols);
    add_waitthread_binary_symbols(dest_dir, &mut symbols);

    symbols
}

fn add_simple_binary_symbols(dest_dir: &PathBuf, symbols: &mut HashMap<&str, String>) {
    let path = dest_dir.join(SIMPLE_BINARY);

    for_each_symbols(&path, |name, value, size| {
        if name == "function1" {
            symbols.insert("SIMPLE_FUNCTION1", format!("0x{:x}", value));
        } else if name == "function2" {
            symbols.insert("SIMPLE_FUNCTION2", format!("0x{:x}", value));
            symbols.insert("SIMPLE_FUNCTION2_LENGTH", size.to_string());
        }
    });
}

fn add_waitthread_binary_symbols(dest_dir: &PathBuf, symbols: &mut HashMap<&str, String>) {
    let path = dest_dir.join(WAITTHREAD_BINARY);

    for_each_symbols(&path, |name, value, _| {
        if name == "breakpoint_func" {
            symbols.insert("THREAD_BREAK_FUNC", format!("0x{:x}", value));
        }
    });
}

fn for_each_symbols<F>(path: &PathBuf, mut func: F)
    where F: FnMut(&String, u64, u64) {

    let mut binary_file = File::open(&path).expect(&format!("Symbol file {:?} error",
                                                           path));
    let mut buffer = vec![];
    binary_file.read_to_end(&mut buffer).unwrap();

    match goblin::parse(&buffer).unwrap() {
        goblin::Object::Elf(elf) => {
            let strtab = &(elf.strtab);
            for sym in &(elf.syms) {
                func(&(strtab.get(sym.st_name).unwrap().to_owned()),
                     sym.st_value,
                     sym.st_size);
            }
        },
        _ => {
            panic!(format!("Unsupported file type"));
        }
    }
}
