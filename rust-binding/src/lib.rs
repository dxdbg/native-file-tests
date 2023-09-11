//
// Copyright (c) 2011-2017, UDI Contributors
// All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
#![deny(warnings)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use goblin::Object;

use pdb::FallibleIterator;

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

const SIMPLE_BINARY_BASE_NAME: &str = "simple-debug-noopt-dynamic";
const WORKERTHREADS_BINARY_BASE_NAME: &str = "workerthreads-debug-noopt-dynamic";

const LINUX_PLATFORM: &str = "linux";
const DARWIN_PLATFORM: &str = "darwin";
const WINDOWS_PLATFORM: &str = "windows";

const N_BNSYM: u8 = 46;
const N_FUN: u8 = 36;
const N_ENSYM: u8 = 78;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to deserialize {path}")]
    FailedDeserialization {
        path: String,

        #[source]
        source: serde_json::Error,
    },

    #[error("{0}")]
    InvalidInput(String),

    #[error("Malformed object file {path}")]
    MalformedObjectFile {
        path: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("Invalid object file {path}: {msg}")]
    InvalidObjectFile { path: String, msg: String },

    #[error("I/O error: {msg}")]
    Io {
        msg: String,
        #[source]
        source: std::io::Error,
    },
}

trait IoErrorExt<T> {
    fn to_io_error(self, msg: &str) -> Result<T, Error>;

    fn map_io_error<M>(self, msg_func: M) -> Result<T, Error>
    where
        M: FnOnce() -> String;
}

trait PdbErrorExt<T> {
    fn map_pdb_error(self, path: &str) -> Result<T, Error>;
}

trait GoblinErrorExt<T> {
    fn map_goblin_error(self, path: &str) -> Result<T, Error>;
}

impl<T> IoErrorExt<T> for Result<T, std::io::Error> {
    fn to_io_error(self, msg: &str) -> Result<T, Error> {
        self.map_err(|e| Error::Io {
            msg: msg.to_owned(),
            source: e,
        })
    }

    fn map_io_error<M>(self, msg_func: M) -> Result<T, Error>
    where
        M: FnOnce() -> String,
    {
        self.map_err(|e: std::io::Error| Error::Io {
            msg: msg_func(),
            source: e,
        })
    }
}

impl<T> PdbErrorExt<T> for Result<T, pdb::Error> {
    fn map_pdb_error(self, path: &str) -> Result<T, Error> {
        self.map_err(|e: pdb::Error| Error::MalformedObjectFile {
            path: path.to_owned(),
            source: Box::new(e),
        })
    }
}

impl<T> GoblinErrorExt<T> for Result<T, goblin::error::Error> {
    fn map_goblin_error(self, path: &str) -> Result<T, Error> {
        self.map_err(|e: goblin::error::Error| Error::MalformedObjectFile {
            path: path.to_owned(),
            source: Box::new(e),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

    compiler: String,
}

#[derive(Debug)]
pub struct TestMetadata {
    simple_path: PathBuf,
    workerthreads_path: PathBuf,
    symbol_addresses: SymbolAddresses,
}

#[derive(Debug)]
struct SymbolAddresses {
    simple_function1_addr: u64,
    simple_function2_addr: u64,
    simple_function2_length: u64,
    thread_break_addr: u64,
    start_notification_addr: u64,
    term_notification_addr: u64,
}

impl TestMetadata {
    pub fn simple_path(&self) -> &Path {
        &self.simple_path
    }

    pub fn workerthreads_path(&self) -> &Path {
        &self.workerthreads_path
    }

    pub fn simple_function1_addr(&self) -> u64 {
        self.symbol_addresses.simple_function1_addr
    }

    pub fn simple_function2_addr(&self) -> u64 {
        self.symbol_addresses.simple_function2_addr
    }

    pub fn simple_function2_length(&self) -> u64 {
        self.symbol_addresses.simple_function2_length
    }

    pub fn thread_break_addr(&self) -> u64 {
        self.symbol_addresses.thread_break_addr
    }

    pub fn start_notification_addr(&self) -> u64 {
        self.symbol_addresses.start_notification_addr
    }

    pub fn term_notification_addr(&self) -> u64 {
        self.symbol_addresses.term_notification_addr
    }
}

pub fn create_test_metadata(nft_path: &Path, rust_os: &str) -> Result<TestMetadata, Error> {
    let platform = convert_to_nft_platform(rust_os);

    let mut simple_paths = None;
    let mut workerthreads_paths = None;

    for entry in nft_path
        .read_dir()
        .to_io_error("Failed to enumerate native-file-tests directory")?
    {
        let path = entry
            .to_io_error("Failed to enumerate directory entry")?
            .path();

        if path.is_file() {
            let file_name = path
                .file_name()
                .ok_or_else(|| {
                    Error::InvalidInput(format!("Invalid file name {}", path.display()))
                })?
                .to_string_lossy()
                .to_string();
            if file_name.contains(".json") {
                let metadata = get_metadata(&path)?;
                if metadata.platform == platform {
                    if metadata.base_name == SIMPLE_BINARY_BASE_NAME {
                        let (exec, debug) =
                            get_symbol_file_names(SIMPLE_BINARY_BASE_NAME, &metadata);
                        simple_paths = Some((nft_path.join(exec), nft_path.join(debug)))
                    } else if metadata.base_name == WORKERTHREADS_BINARY_BASE_NAME {
                        let (exec, debug) =
                            get_symbol_file_names(WORKERTHREADS_BINARY_BASE_NAME, &metadata);
                        workerthreads_paths = Some((nft_path.join(exec), nft_path.join(debug)))
                    }
                }
            }
        }
    }

    let simple_paths = simple_paths.ok_or_else(|| {
        Error::InvalidInput(format!(
            "Failed to find simple binaries for platform {}",
            platform
        ))
    })?;
    let workerthreads_paths = workerthreads_paths.ok_or_else(|| {
        Error::InvalidInput(format!(
            "Failed to find workerthreads binaries for platform {}",
            platform
        ))
    })?;

    let symbol_addresses = build_symbol_addresses(
        platform,
        (&simple_paths.0, &simple_paths.1),
        (&workerthreads_paths.0, &workerthreads_paths.1),
    )?;

    Ok(TestMetadata {
        simple_path: simple_paths.0,
        workerthreads_path: workerthreads_paths.0,
        symbol_addresses,
    })
}

fn convert_to_nft_platform(rust_os: &str) -> &str {
    match rust_os {
        "linux" => LINUX_PLATFORM,
        "macos" => DARWIN_PLATFORM,
        "windows" => WINDOWS_PLATFORM,
        _ => panic!("Unsupported platform"),
    }
}

fn get_symbol_file_names(prefix: &str, metadata: &NativeFileMetadata) -> (String, String) {
    match metadata.debug_hash.as_ref() {
        Some(debug_hash) => (
            prefix.to_owned() + &metadata.exec_suffix + "." + &metadata.exec_hash,
            prefix.to_owned() + &metadata.exec_suffix + ".debug." + debug_hash,
        ),
        None => {
            let exec = prefix.to_owned() + &metadata.exec_suffix + "." + &metadata.exec_hash;
            (exec.clone(), exec)
        }
    }
}

fn get_metadata(json_path: &Path) -> Result<NativeFileMetadata, Error> {
    let mut json_file = File::open(json_path).map_io_error(|| {
        format!(
            "Failed to open native-file-tests metadata file {}",
            json_path.display()
        )
    })?;

    serde_json::from_reader(&mut json_file).map_err(|e| Error::FailedDeserialization {
        path: format!("{}", json_path.display()),
        source: e,
    })
}

fn build_symbol_addresses(
    platform: &str,
    simple_paths: (&Path, &Path),
    workerthreads_paths: (&Path, &Path),
) -> Result<SymbolAddresses, Error> {
    let mut symbol_addresses = SymbolAddresses {
        simple_function1_addr: 0,
        simple_function2_addr: 0,
        simple_function2_length: 0,
        thread_break_addr: 0,
        start_notification_addr: 0,
        term_notification_addr: 0,
    };

    add_simple_binary_symbols(
        platform,
        simple_paths.0,
        simple_paths.1,
        &mut symbol_addresses,
    )?;
    add_workerthreads_binary_symbols(
        platform,
        workerthreads_paths.0,
        workerthreads_paths.1,
        &mut symbol_addresses,
    )?;

    Ok(symbol_addresses)
}

fn add_simple_binary_symbols(
    platform: &str,
    binary_path: &Path,
    debug_sym_path: &Path,
    symbol_addresses: &mut SymbolAddresses,
) -> Result<(), Error> {
    for_each_symbols(
        platform,
        binary_path,
        debug_sym_path,
        |name, value, size| {
            if name == "function1" {
                symbol_addresses.simple_function1_addr = value;
            } else if name == "function2" {
                symbol_addresses.simple_function2_addr = value;
                symbol_addresses.simple_function2_length = size;
            }
        },
    )
}

fn add_workerthreads_binary_symbols(
    platform: &str,
    binary_path: &Path,
    debug_sym_path: &Path,
    symbol_addresses: &mut SymbolAddresses,
) -> Result<(), Error> {
    for_each_symbols(
        platform,
        binary_path,
        debug_sym_path,
        |name: &str, value, _| {
            match name {
                "breakpoint_thr_func" => {
                    symbol_addresses.thread_break_addr = value;
                }
                "start_notification" => {
                    symbol_addresses.start_notification_addr = value;
                }
                "term_notification" => {
                    symbol_addresses.term_notification_addr = value;
                }
                _ => {}
            };
        },
    )
}

fn for_each_symbols<F>(
    platform: &str,
    binary_path: &Path,
    debug_sym_path: &Path,
    func: F,
) -> Result<(), Error>
where
    F: FnMut(&str, u64, u64),
{
    match platform {
        WINDOWS_PLATFORM => for_each_symbols_from_pdb(binary_path, debug_sym_path, func),
        _ => for_each_symbols_from_obj_file(binary_path, func),
    }
}

fn for_each_symbols_from_obj_file<F>(path: &Path, mut func: F) -> Result<(), Error>
where
    F: FnMut(&str, u64, u64),
{
    let path_str = format!("{}", path.display());

    let mut binary_file =
        File::open(path).map_io_error(|| format!("Failed to open {}", path.display()))?;
    let mut buffer = vec![];
    binary_file
        .read_to_end(&mut buffer)
        .map_io_error(|| format!("Failed to read file {}", path.display()))?;

    match Object::parse(&buffer).map_goblin_error(&path_str)? {
        Object::Elf(elf) => {
            let strtab = &(elf.strtab);
            for sym in &(elf.syms) {
                func(
                    strtab
                        .get_at(sym.st_name)
                        .ok_or_else(|| Error::InvalidObjectFile {
                            path: path_str.to_string(),
                            msg: "Failed to get symbol name".to_owned(),
                        })?,
                    sym.st_value,
                    sym.st_size,
                );
            }
        }
        Object::Mach(goblin::mach::Mach::Binary(mach)) => {
            let syms = mach.symbols.ok_or_else(|| Error::InvalidObjectFile {
                path: path_str.to_string(),
                msg: "Failed to get symbols".to_owned(),
            })?;

            let mut addr = 0;
            let mut sym_name = "".to_owned();
            for sym_result in syms.iter() {
                let (name, nlist) = sym_result.map_goblin_error(&path_str)?;

                match nlist.n_type {
                    N_BNSYM => {
                        addr = nlist.n_value;
                    }
                    N_FUN => {
                        if nlist.n_strx > 1 {
                            // remove leading _ from symbol name
                            sym_name = name.split_at(1).1.to_owned();
                        }
                    }
                    N_ENSYM => {
                        func(&sym_name, addr, nlist.n_value);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            panic!("Unsupported file type for file {:?}", path);
        }
    }

    Ok(())
}

fn for_each_symbols_from_pdb<F>(
    binary_path: &Path,
    debug_sym_path: &Path,
    mut func: F,
) -> Result<(), Error>
where
    F: FnMut(&str, u64, u64),
{
    let binary_path_str = format!("{}", binary_path.display());
    let debug_sym_path_str = format!("{}", debug_sym_path.display());

    let mut binary_file =
        File::open(binary_path).map_io_error(|| format!("Failed to open {}", binary_path_str))?;
    let mut buffer = vec![];
    binary_file
        .read_to_end(&mut buffer)
        .map_io_error(|| format!("Failed to read file {}", binary_path_str))?;

    let image_base;
    let addrs: Vec<u32> = match Object::parse(&buffer).map_goblin_error(&binary_path_str)? {
        Object::PE(pe) => {
            image_base = pe
                .header
                .optional_header
                .ok_or_else(|| Error::InvalidObjectFile {
                    path: binary_path_str.to_string(),
                    msg: "Missing optional header".to_owned(),
                })?
                .windows_fields
                .image_base;
            pe.sections.iter().map(|s| s.virtual_address).collect()
        }
        _ => {
            return Err(Error::InvalidInput(format!(
                "Unsupported file type for file {}",
                binary_path_str
            )))
        }
    };

    let pdb_file = File::open(debug_sym_path)
        .map_io_error(|| format!("Failed to open {}", debug_sym_path_str))?;
    let mut pdb = pdb::PDB::open(pdb_file).map_pdb_error(&debug_sym_path_str)?;
    let addr_map = pdb.address_map().map_pdb_error(&debug_sym_path_str)?;
    let dbi = pdb.debug_information().map_pdb_error(&debug_sym_path_str)?;
    let mut modules = dbi.modules().map_pdb_error(&debug_sym_path_str)?;
    while let Some(module) = modules.next().map_pdb_error(&debug_sym_path_str)? {
        if let Some(module_info) = pdb
            .module_info(&module)
            .map_pdb_error(&debug_sym_path_str)?
        {
            let mut symbol_iter = module_info.symbols().map_pdb_error(&debug_sym_path_str)?;
            while let Some(symbol) = symbol_iter.next().map_pdb_error(&debug_sym_path_str)? {
                if let Ok(pdb::SymbolData::Procedure(proc_sym)) = symbol.parse() {
                    let section_offset =
                        proc_sym
                            .offset
                            .to_section_offset(&addr_map)
                            .ok_or_else(|| Error::InvalidObjectFile {
                                path: debug_sym_path_str.to_string(),
                                msg: "Failed to convert offset to section offset".to_owned(),
                            })?;
                    let seg_addr = addrs[(section_offset.section - 1) as usize];
                    let sym_addr = image_base + (seg_addr as u64) + (proc_sym.offset.offset as u64);
                    let sym_name = proc_sym.name.to_string();
                    func(sym_name.as_ref(), sym_addr, proc_sym.len as u64);
                }
            }
        }
    }

    Ok(())
}
