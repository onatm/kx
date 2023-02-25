use clap::Parser;
use kx::Config;
use skim::prelude::*;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Switches to context <NAME>
    #[arg()]
    name: Option<String>,
    /// Shows the current context
    #[arg(short = 'c', long = "current")]
    current: bool,
    /// Unsets the current context
    #[arg(short = 'u', long = "unset")]
    unset: bool,
}

fn main() {
    let args = Args::parse();

    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("error: cannot find HOME directory");
        process::exit(1);
    });

    let config_path = &home_dir.join(".kube").join("config");

    let contents = &fs::read_to_string(config_path).unwrap_or_else(|_| {
        eprintln!("error: cannot read kube config");
        process::exit(1);
    });

    let contents = contents.lines().collect::<Vec<&str>>();

    let config = &mut Config::load(contents);

    if args.current {
        let current_context = config.get_current_context().unwrap_or_else(|err| {
            eprintln!("error: {}", err);
            process::exit(1);
        });

        println!("{}", current_context);
        return;
    }

    if args.unset {
        unset_current_context(config_path, config);

        println!("current-context unset");
        return;
    }

    if let Some(selected_context) = args.name {
        match config.check_context(&selected_context) {
            Ok(res) => {
                if !res {
                    eprintln!(
                        "error: no context exists with the name: \"{}\"",
                        selected_context
                    );
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
                process::exit(1);
            }
        };

        let new_context = format!("current-context: {}", &selected_context);
        set_current_context(config_path, config, &new_context);

        println!("Switched to context \"{}\"", selected_context);

        return;
    }

    let contexts = config.list_contexts().unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        process::exit(1);
    });

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("error: cannot show contexts");
            eprintln!("{e}");
            process::exit(1);
        });

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(contexts));

    let output = Skim::run_with(&options, Some(items)).unwrap();

    if output.is_abort {
        println!("context is not changed");
        process::exit(0);
    }

    let selected_context = output
        .selected_items
        .first()
        .unwrap_or_else(|| {
            eprintln!("error: cannot pick context");
            process::exit(1);
        })
        .output();

    let new_context = format!("current-context: {}", &selected_context);
    set_current_context(config_path, config, &new_context);

    println!("Switched to context \"{}\"", selected_context);
}

fn set_current_context<'a>(config_path: &'a PathBuf, config: &'a Config<'a>, new_context: &'a str) {
    config.set_current_context(new_context).unwrap_or_else(|_| {
        eprintln!("error: cannot set current-context");
        process::exit(1);
    });

    fs::write(config_path, config.get_config()).unwrap_or_else(|_| {
        eprintln!("error: cannot save kube config");
        process::exit(1);
    });
}

fn unset_current_context<'a>(config_path: &'a PathBuf, config: &'a Config<'a>) {
    config.unset_current_context().unwrap_or_else(|_| {
        eprintln!("error: cannot unset current-context");
        process::exit(1);
    });

    fs::write(config_path, config.get_config()).unwrap_or_else(|_| {
        eprintln!("error: cannot save kube config");
        process::exit(1);
    });
}
