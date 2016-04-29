extern crate getopts;
extern crate glob;
extern crate notify;

use getopts::Options;
use glob::Pattern;
use notify::{RecommendedWatcher, Watcher, Event};
use std::env;
use std::process::Command;
use std::sync::mpsc::channel;

struct ProgramOpts {
  help: bool,
  only_paths: Vec<String>,
  exclude_paths: Vec<String>,
  command: Vec<String>
}

fn print_usage(program: &str, opts: &Options) {
  let brief = format!("Usage: {} [options] COMMAND", program);
  print!("{}", opts.usage(&brief));
}

fn program_options() -> Options {
  let mut opts = Options::new();

  opts.optmulti("o", "only", "only run COMMAND when path that changed matches PATH", "PATH");
  opts.optmulti("e", "exclude", "run COMMAND when path doesn't match PATH", "PATH");
  opts.optflag("h", "help", "print this help menu");

  return opts;
}

fn read_program_options(opts: &Options, args: Vec<String>) -> ProgramOpts {
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) }
  };

  let mut exclude_paths = matches.opt_strs("e");
  exclude_paths.push("**/.*/*".to_string());
  exclude_paths.push("**/.*".to_string());

  let mut only_paths = matches.opt_strs("o");

  if only_paths.len() == 0 {
    only_paths.push(".".to_string());
  }

  return ProgramOpts {
    help: matches.opt_present("h"),
    only_paths: only_paths,
    exclude_paths: exclude_paths,
    command: matches.free
  }
}

fn main() {
  let opts = program_options();
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let program_options = read_program_options(&opts, args);

  if program_options.help || program_options.command.is_empty() {
    print_usage(&program, &opts);
    return;
  }

  let (tx, rx) = channel();

  let mut watcher: RecommendedWatcher = Watcher::new(tx).expect("Wesley does not support watching on your platform.");

  for only_path in program_options.only_paths {
    let _ = watcher.watch(only_path);
  }

  let (command_name, args) = program_options.command.split_first().unwrap();

  let mut command = Command::new(command_name);
  command.args(args);

  loop {
    let result = rx.recv();

    match result {
      Ok(Event{op: Ok(_), path: Some(path)}) => {
        let matching_exclude = program_options.exclude_paths.iter()
          .map(|exclude| {
            let exclude_path = exclude;

            let mut absolute_path = std::env::current_dir().unwrap();
            absolute_path.push(exclude_path);
            absolute_path
          })
          .filter(|exclude| {
            Pattern::new(exclude.as_path().to_str().unwrap()).unwrap().matches(path.as_path().to_str().unwrap())
          })
          .nth(0);

        if matching_exclude.is_none() {
          println!("- changed {:?}", path);
          println!("- running {:?}", command);
          let _ = command.status();
          println!("");
        }
      },
      _ => ()
    }
  }
}
