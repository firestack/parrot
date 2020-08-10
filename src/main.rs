use cli::Command;
use error::unwrap_log;

mod cli;
mod cmd;
mod data;
mod error;

fn main() {
    let config = cli::parse();
    let mut data = unwrap_log(data::DataManager::new(config.path));
    match config.cmd {
        Command::Init {} => {
            unwrap_log(data.initialize());
            println!("Parrot has been initialized.")
        }
        Command::Add { cmd, name } => {
            let snap = unwrap_log(cmd::execute(&cmd));
            let name = if let Some(name) = name {
                name
            } else {
                String::from("default")
            };
            unwrap_log(data.add_snapshot(&cmd, &name, &snap.stdout));
        }
        Command::Run {} => {
            let mut success = true;
            let snaps = unwrap_log(data.get_all_snapshots());
            for snap in &snaps {
                let result = unwrap_log(cmd::execute(&snap.cmd));
                if result.stdout != snap.content {
                    success = false;
                    println!("Test failed for '{}'.", &snap.cmd);
                }
            }
            if success {
                println!("Success !");
            } else {
                println!("Failure...");
            }
        }
    }
}
