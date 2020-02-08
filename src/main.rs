extern crate clap;
extern crate dirs;

use clap::{App, Arg};
use kx::Config;
use skim::{Skim, SkimOptionsBuilder};
use std::fs;
use std::io::Cursor;
use std::process;

fn main() {
    let matches = App::new("kx")
        .version("1.0")
        .author("onatm - https://github.com/onatm")
        .about("Interactively switch between kubernetes contexts without any external dependencies")
        .arg(Arg::with_name("NAME").help("Switch to context <NAME>"))
        .arg(
            Arg::with_name("current")
                .short("c")
                .long("current")
                .help("Show the current context"),
        )
        .get_matches();

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

    if let Some(new_context) = matches.value_of("NAME") {
        println!("new context {}", new_context);
    }

    if matches.is_present("current") {
        let current_context = config.get_current_context().unwrap_or_else(|err| {
            eprintln!("error: {}", err);
            process::exit(1);
        });

        println!("{}", current_context);
    }

    let contexts = config.list_contexts();

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(contexts))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected_context = selected_items
        .iter()
        .next()
        .unwrap_or_else(|| {
            println!("context is not changed");
            process::exit(1);
        })
        .get_output_text();

    let new_context = format!("current-context: {}", &selected_context);

    config
        .set_current_context(&new_context)
        .unwrap_or_else(|_| {
            eprintln!("error: cannot set current-context");
            process::exit(1);
        });

    fs::write(config_path, config.get_config()).unwrap_or_else(|_| {
        eprintln!("error: cannot save kube config");
        process::exit(1);
    });

    println!("Switched to context \"{}\"", selected_context);
}
