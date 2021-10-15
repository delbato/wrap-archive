extern crate wrap;

extern crate clap;
extern crate cli_table;
extern crate globwalk;

use std::fs::File;
use std::path::PathBuf;
use std::env;

use clap::{
    AppSettings,
    Clap,
};
use cli_table::format::Justify;
use cli_table::{
    print_stdout,
    Cell,
    Style,
    Table,
};
use wrap::Archive;

macro_rules! int_len {
    ($int:ident) => {
        {
            $int.to_string().len()
        }
    };
}

macro_rules! unit {
    ($int:ident) => {
        {
            if int_len!($int) >= 11 {
                "GiB"
            } else if int_len!($int) >= 8 {
                "MiB"
            } else if int_len!($int) >= 5 {
                "KiB"
            } else {
                "B"
            }
        }
    };
}

macro_rules! size_format {
    ($int:ident, $unit:ident) => {
        {
            let size = match $unit {
                "GiB" => (($int / 1024) / 1024) / 1024,
                "MiB" => ($int / 1024) / 1024,
                "KiB" => $int / 1024,
                _ => $int
            };
            format!("{}{}", size, $unit)
        }
    };
}

#[derive(Clap)]
#[clap(name = "wrap", about = "CLI utility for handling .wrap archive files")]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Daniel W. <delbato@pm.me>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct RunArgs {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Pack(PackArgs),
    Unpack(UnpackArgs),
    Append(AppendArgs),
    List(ListArgs),
}

#[derive(Clap)]
#[clap(about = "Packs content into a new .wrap archive file")]
struct PackArgs {
    #[clap(index = 1, required = true, about = "Paths to pack")]
    input: Vec<PathBuf>,
    #[clap(short, long, about = "Path to the .wrap archive")]
    output: Option<PathBuf>,
    #[clap(short, long, about = "Recurse through the input directory")]
    recursive: bool,
}

#[derive(Clap)]
#[clap(about = "Unpacks contents of an .wrap archive file")]
struct UnpackArgs {}

#[derive(Clap)]
#[clap(about = "Lists contents of an .wrap archive file")]
struct ListArgs {
    #[clap(index = 1, about = "Path to the .wrap file to check")]
    archive: PathBuf,
    #[clap(long, short, about = "Print additional information in a table")]
    table: bool
}

#[derive(Clap)]
#[clap(about = "Adds content to an existing .wrap archive file")]
struct AppendArgs {
    #[clap(index = 1, required = true, about = "Paths to pack")]
    input: Vec<PathBuf>,
    #[clap(short, long, about = "Path to the .wrap archive")]
    output: PathBuf,
    #[clap(short, long, about = "Recurse through the input directory")]
    recursive: bool,
}

fn main() {
    let run_args = RunArgs::parse();

    match run_args.command {
        SubCommand::List(list_args) => subcmd_list(list_args),
        SubCommand::Pack(pack_args) => subcmd_pack(pack_args),
        _ => {}
    };
}

fn subcmd_list(list_args: ListArgs) {
    match list_args.table {
        true => list_pretty(list_args),
        false => list_simple(list_args)
    };
}

fn list_pretty(list_args: ListArgs) {
    let file = File::open(&list_args.archive).unwrap();
    let mut archive = Archive::new(file).unwrap();
    let file_list = archive.get_file_list();
    let mut table_vec = vec![];
    for file in file_list {
        let file_info = archive.get_file_info(&file).unwrap();
        let raw_size = file_info.raw_size;
        let compressed_size = file_info.data_size;
        let unit_raw = unit!(raw_size);
        let unit_compressed = unit!(compressed_size);
        let raw_size_str = size_format!(raw_size, unit_raw);
        let compressed_size_str = size_format!(compressed_size, unit_compressed);
        let row = vec![file.cell(), compressed_size_str.cell(), raw_size_str.cell()];
        table_vec.push(row);
    }
    let table = table_vec
        .table()
        .title(vec!["FILENAME".cell(), "COMPRESSED SIZE".cell(), "ACTUAL SIZE".cell()])
        .bold(true);

    print_stdout(table).unwrap();
}

fn list_simple(list_args: ListArgs) {
    let file = File::open(&list_args.archive).unwrap();
    let archive = Archive::new(file).unwrap();
    let file_list = archive.get_file_list();
    file_list.into_iter().for_each(|file| println!(">  {}", file));
}

fn subcmd_pack(pack_args: PackArgs) {
    if pack_args.recursive && pack_args.input.len() > 1 {
        eprintln!("-r option is not compatible with multiple inputs!");
        return;
    }
}
