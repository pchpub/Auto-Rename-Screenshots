use auto_rename_screenshots::mods::{
    cliinput::Command,
    config::{add, remove},
    file::rename,
    types::{Config, CONFIG, DEBUG_MODE},
};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let opt = Command::from_args();
    {
        let config_path = match &opt {
            Command::Add { config, .. } => config,
            Command::Remove { config, .. } => config,
            Command::Rename { config, .. } => config,
        };
        let debug_mode = match &opt {
            Command::Add { debug, .. } => debug,
            Command::Remove { debug, .. } => debug,
            Command::Rename { debug, .. } => debug,
        };
        *DEBUG_MODE.lock().await = *debug_mode;
        *CONFIG.lock().await = Config::from_file(config_path).await.unwrap();
    }

    match &opt {
        Command::Add {
            debug: _,
            config: _,
            data,
        } => match add(&opt, data).await {
            Ok(data) => {
                println!("[OK] {}", data);
            }
            Err(err) => {
                println!("[ERROR] {:?}", err);
            }
        },
        Command::Remove {
            debug: _,
            config: _,
            data,
        } => match remove(&opt, data).await {
            Ok(data) => {
                println!("[OK] {}", data);
            }
            Err(err) => {
                println!("[ERROR] {:?}", err);
            }
        },
        Command::Rename {
            debug: _,
            input,
            output,
            config: _,
            compress_type,
        } => match rename(&opt, input, output, compress_type).await {
            Ok(data) => {
                println!("[OK] {}", data);
            }
            Err(err) => {
                println!("[ERROR] {:?}", err);
            }
        },
    }

    // println!("{:?}", opt);
}
