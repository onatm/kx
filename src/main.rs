extern crate clap;
extern crate dirs;

use clap::{App, Arg};
use kx::Config;
use skim::{Skim, SkimOptionsBuilder};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
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

    if matches.is_present("current") {
        let current_context = config.get_current_context().unwrap_or_else(|err| {
            eprintln!("error: {}", err);
            process::exit(1);
        });

        println!("{}", current_context);
        return;
    }

    if let Some(selected_context) = matches.value_of("NAME") {
        if !config.check_context(selected_context) {
            eprintln!(
                "error: no context exists with the name: \"{}\"",
                selected_context
            );
            process::exit(1);
        }

        let new_context = format!("current-context: {}", &selected_context);
        set_current_context(config_path, config, &new_context);

        println!("Switched to context \"{}\"", selected_context);

        return;
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
            process::exit(0);
        })
        .get_output_text();

    let new_context = format!("current-context: {}", &selected_context);
    set_current_context(config_path, config, &new_context);

    println!("Switched to context \"{}\"", selected_context);
}

fn set_current_context<'a>(config_path: &'a PathBuf, config: &'a Config<'a>, new_context: &'a str) {
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
}
