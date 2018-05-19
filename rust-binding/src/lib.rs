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
extern crate pdb;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;

use goblin::Object;

use pdb::FallibleIterator;

use serde::Deserialize;
use serde_json::Deserializer;

const SIMPLE_BINARY_BASE_NAME: &'static str = "simple-debug-noopt-dynamic";
const WORKERTHREADS_BINARY_BASE_NAME: &'static str = "workerthreads-debug-noopt-dynamic";

const LINUX_PLATFORM: &'static str = "linux";
const DARWIN_PLATFORM: &'static str = "darwin";
const WINDOWS_PLATFORM: &'static str = "windows";

const N_BNSYM: u8 = 46;
const N_FUN: u8 = 36;
const N_ENSYM: u8 = 78;

#[derive(Debug,Serialize,Deserialize)]
struct NativeFileMetadata {

    #[serde(rename = "configName")]
    config_name: String,

    #[serde(rename = "baseName")]
    base_name: String,

    #[serde(rename = "objectSha256s")]
    objects: HashMap<String, String>,

    #[serde(rename = "objectSuffix")]
    obj_suffix: String,

    #[serde(rename = "executableSha256")]
    exec_hash: String,

    #[serde(rename = "executableSuffix")]
    exec_suffix: String,

    #[serde(rename = "debugSha256")]
    debug_hash: Option<String>,

    machine: String,

    platform: String,

    flags: HashMap<String, String>,

    compiler: String
}

pub fn setup(nft_path: &PathBuf,
             out_path: &PathBuf,
             rust_os: &str) {

    let platform = convert_to_nft_platform(rust_os);

    let mut simple_path_opt = None;
    let mut workerthreads_path_opt = None;

    for entry in nft_path.read_dir()
                         .expect("Failed to enumerate native-file-tests directory") {
        let path = entry.expect("Failed to enumerate directory entry").path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            if file_name.contains(".json") {
                let metadata = get_metadata(&path);
                if metadata.platform == platform {
                    if metadata.base_name == SIMPLE_BINARY_BASE_NAME {
                        let (exec, debug) = get_symbol_file_names(SIMPLE_BINARY_BASE_NAME,
                                                                  &metadata);
                        simple_path_opt = Some( ( nft_path.join(exec), nft_path.join(debug) ) )
                    } else if metadata.base_name == WORKERTHREADS_BINARY_BASE_NAME {
                        let (exec, debug) = get_symbol_file_names(WORKERTHREADS_BINARY_BASE_NAME,
                                                                  &metadata);
                        workerthreads_path_opt = Some( ( nft_path.join(exec), nft_path.join(debug) ) )
                    }
                }
            }
        }
    }

    let simple_paths = simple_path_opt.expect("Path to simple binary was not set");
    let workerthreads_paths = workerthreads_path_opt.expect("Path to workerthreads binary was not set");

    let mod_file_path = out_path.join("native_file_tests.rs");
    let mut mod_file = File::create(&mod_file_path).unwrap();

    let symbols = build_symbols(platform, &simple_paths, &workerthreads_paths);

    let sym_defs = symbols.iter()
                          .map(|e| format!("pub const {}: u64 = {};\n", e.0, e.1))
                          .fold("".to_owned(), |acc, elem| acc + &elem);

    let mod_file_content = format!("
pub const SIMPLE_EXEC_PATH: &'static str = \"{}\";
pub const WORKERTHREADS_EXEC_PATH: &'static str = \"{}\";

{}
",
    simple_paths.0.to_str().unwrap(),
    workerthreads_paths.0.to_str().unwrap(),
    sym_defs);

    mod_file.write_all(&mod_file_content.into_bytes())
            .expect("Failed to write native file tests module");
}

fn convert_to_nft_platform(rust_os: &str) -> &str {
    match rust_os {
        "linux" => LINUX_PLATFORM,
        "macos" => DARWIN_PLATFORM,
        "windows" => WINDOWS_PLATFORM,
        _ => panic!(format!("Unsupported platform"))
    }
}

fn get_symbol_file_names(prefix: &str,
                         metadata: &NativeFileMetadata) -> (String, String) {

    match metadata.debug_hash.as_ref() {
        Some(debug_hash) => {
            (
                prefix.to_owned() + &metadata.exec_suffix + "." + &metadata.exec_hash,
                prefix.to_owned() + &metadata.exec_suffix + ".debug." + debug_hash
            )
        },
        None => {
            let exec = prefix.to_owned() + &metadata.exec_suffix + "." + &metadata.exec_hash;
            (exec.clone(), exec)
        }
    }
}

fn get_metadata(json_path: &PathBuf) -> NativeFileMetadata {
    let mut json_file = File::open(json_path).unwrap();

    let mut de = Deserializer::from_reader(&mut json_file);

    Deserialize::deserialize(&mut de).unwrap()
}

fn build_symbols(platform: &str,
                 simple_paths: &(PathBuf, PathBuf),
                 workerthreads_paths: &(PathBuf, PathBuf) ) -> HashMap<&'static str, String> {
    let mut symbols = HashMap::new();

    add_simple_binary_symbols(platform, simple_paths, &mut symbols);
    add_workerthreads_binary_symbols(platform, workerthreads_paths, &mut symbols);

    symbols
}

fn add_simple_binary_symbols(platform: &str,
                             paths: &(PathBuf, PathBuf),
                             symbols: &mut HashMap<&'static str, String>) {

    for_each_symbols(platform, paths, |name, value, size| {
        if name == "function1" {
            symbols.insert("SIMPLE_FUNCTION1", format!("0x{:x}", value));
        } else if name == "function2" {
            symbols.insert("SIMPLE_FUNCTION2", format!("0x{:x}", value));
            symbols.insert("SIMPLE_FUNCTION2_LENGTH", size.to_string());
        }
    });
}

fn add_workerthreads_binary_symbols(platform: &str,
                                 path: &(PathBuf, PathBuf),
                                 symbols: &mut HashMap<&'static str, String>) {

    for_each_symbols(platform, path, |name, value, _| {
        match name.as_str() {
            "breakpoint_thr_func" => {
                symbols.insert("THREAD_BREAK_FUNC", format!("0x{:x}", value));
            },
            "start_notification" => {
                symbols.insert("START_NOTIFICATION_FUNC", format!("0x{:x}", value));
            },
            "term_notification" => {
                symbols.insert("TERM_NOTIFICATION_FUNC", format!("0x{:x}", value));
            },
            _ => {}
        };
    });
}

fn for_each_symbols<F>(platform: &str,
                       paths: &(PathBuf, PathBuf),
                       func: F)
    where F: FnMut(&String, u64, u64) {

    match platform {
        WINDOWS_PLATFORM => for_each_symbols_from_pdb(paths, func),
        _ => for_each_symbols_from_obj_file(&paths.0, func)
    }

}

fn for_each_symbols_from_obj_file<F>(path: &PathBuf,
                                     mut func: F) where F: FnMut(&String, u64, u64) {

    let mut binary_file = File::open(&path).expect(&format!("Symbol file {:?} error",
                                                            path));
    let mut buffer = vec![];
    binary_file.read_to_end(&mut buffer).unwrap();

    match Object::parse(&buffer).unwrap() {
        Object::Elf(elf) => {
            let strtab = &(elf.strtab);
            for sym in &(elf.syms) {
                func(&(strtab.get(sym.st_name).unwrap().unwrap().to_owned()),
                     sym.st_value,
                     sym.st_size);
            }
        },
        Object::Mach(goblin::mach::Mach::Binary(mach)) => {
            let syms = mach.symbols.unwrap();

            let mut addr = 0;
            let mut sym_name = "".to_owned();
            for sym_result in syms.iter() {
                let (name, nlist) = sym_result.unwrap();

                match nlist.n_type {
                    N_BNSYM => {
                        addr = nlist.n_value;
                    },
                    N_FUN => {
                        if nlist.n_strx > 1 {
                            // remove leading _ from symbol name
                            sym_name = name.split_at(1).1.to_owned();
                        }
                    },
                    N_ENSYM => {
                        func(&sym_name, addr, nlist.n_value);
                    }
                    _ => {}
                }
            }
        },
        _ => {
            panic!(format!("Unsupported file type for file {:?}", path));
        }
    }
}

fn for_each_symbols_from_pdb<F>(paths: &(PathBuf, PathBuf),
                                mut func: F) where F: FnMut(&String, u64, u64) {

    let mut binary_file = File::open(&paths.0).expect(&format!("Symbol file {:?} error",
                                                              paths.0));
    let mut buffer = vec![];
    binary_file.read_to_end(&mut buffer).unwrap();

    let image_base;
    let addrs: Vec<u32> = match Object::parse(&buffer).unwrap() {
        Object::PE(pe) => {
            image_base = pe.header.optional_header.unwrap().windows_fields.image_base;
            pe.sections.iter()
                .map(|s| s.virtual_address)
                .collect()
        },
        _ => {
            panic!(format!("Unsupported file type for file {:?}", paths.0))
        }
    };

    let pdb_file = std::fs::File::open(&paths.1).unwrap();
    let mut pdb = pdb::PDB::open(pdb_file).unwrap();

    let dbi = pdb.debug_information().unwrap();
    let mut modules = dbi.modules().unwrap();
    while let Some(module) = modules.next().unwrap() {
        let module_info = pdb.module_info(&module).unwrap();
        let mut symbol_iter = module_info.symbols().unwrap();
        while let Some(symbol) = symbol_iter.next().unwrap() {
            match symbol.parse() {
                Ok(symbol_data) => {
                    match symbol_data {
                        pdb::SymbolData::Procedure(data) => {
                            let seg_addr = addrs[(data.segment-1) as usize];
                            let sym_addr = image_base + (seg_addr as u64) + (data.offset as u64);
                            let sym_name = symbol.name().unwrap().to_string().into_owned();
                            func(&sym_name,
                                 sym_addr,
                                 data.len as u64);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

