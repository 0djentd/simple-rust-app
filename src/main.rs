use clap::{Parser, Subcommand};
use std::fmt::Debug;
mod data;
use data::{Entry, EntriesList};

#[derive(Parser, Debug)]
#[clap(name = "SIMPLE_RUST_APP")]
#[clap(author = "0djentd")]
#[clap(version = "0.1")]
struct ArgsCli {
    ///Subcommand
    #[clap(subcommand)]
    command: Option<Commands>,

    ///Additional info
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,

    #[clap(long)]
    debug: bool,

    ///File to use as config
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///List all tasks
    List {
        ///Maximum entries to display
        #[clap(default_value = "5")]
        count: u32,
    },
    ///Add new task
    Add {
        ///Task text
        data: Option<String>,
    },
    ///Remove task
    Remove {
        ///Task index
        number: u32,
    },
}

fn add(args: &ArgsCli, mut e: EntriesList) {
    let mut tempfile_path = dirs::home_dir().unwrap();
    tempfile_path.push(".SIMPLE_RUST_APP_E");
    let tempfile_path_str = tempfile_path.to_str().unwrap();
    std::fs::write(&tempfile_path, String::from("")).unwrap();
    std::process::Command::new("vim").arg(tempfile_path_str).status().unwrap();
    println!("{}", tempfile_path_str);
    let data = std::fs::read_to_string(&tempfile_path).unwrap();
    std::fs::remove_file(&tempfile_path).unwrap();
    let n = Entry {
        data: data,
    };
    e.data.push(n);
    save_entries(&args, &e).unwrap();
}

fn list(args: &ArgsCli, e: EntriesList, number: u32) {
    let mut i = 0;
    if args.verbose > 0 {
        println!("Displaying {} entries", number);
    }
    for x in &e.data {
        println!("{} {:?}", i, x.data);
        i = i + 1;
        if i == number {
            let y = &e.data.len() - number as usize;
            if y > 0 {
                println!("... {} more", y);
            }
            break
        }
    }
}

fn remove(args: &ArgsCli, mut e: EntriesList, number: u32) {
    let mut i: u32 = 0;
    for _ in e.data.clone() {
        if i == number {
            e.data.remove(i.try_into().unwrap());
        }
        i = i + 1;
    }
    save_entries(&args, &e).unwrap();
}

fn load_entries(args: &ArgsCli) -> EntriesList {
    let path = match &args.config {
        Some(val) => String::from(val),
        None => {
            let mut path = dirs::home_dir().unwrap();
            path.push(".simple-rust-app");
            String::from(path.to_str().unwrap())
        }
    };
    let f = match std::fs::read_to_string(&path) {
        Ok(val) => val,
        Err(_) => {
            let e = serde_json::to_string(&EntriesList{data: vec![]}).unwrap();
            std::fs::write(&path, e).unwrap();
            std::fs::read_to_string(&path).unwrap()
        }
    };
    serde_json::from_str(&f).unwrap()
}

fn save_entries(args: &ArgsCli, e: &EntriesList) -> Result<(), serde_json::Error> {
    let s = serde_json::to_string(&e)?;
    let path = match &args.config {
        Some(val) => String::from(val),
        None => {
            let mut path = dirs::home_dir().unwrap();
            path.push(".simple-rust-app");
            String::from(path.to_str().unwrap())
        }
    };
    std::fs::write(path, s).unwrap();
    Ok(())
}

fn main() {
    let cli = ArgsCli::parse();
    let e = load_entries(&cli);
    match cli.command{
        Some(Commands::Add{..}) => add(&cli, e),
        Some(Commands::List{count}) => list(&cli, e, count),
        Some(Commands::Remove{number}) => remove(&cli, e, number.clone()),
        None => list(&cli, e, 5),
    }
}
