use super::{
    cliinput::Command,
    file::save_file,
    types::{Config, PCHStdError},
};
use crate::{mods::types::CONFIG, ok, warn};

pub async fn add(opt: &Command, data: &Option<Vec<String>>) -> Result<String, PCHStdError> {
    if let Some(data) = data {
        for data in data.iter() {
            CONFIG.lock().await.file_names.push(data.to_string());
            ok!("Added:", data);
        }
        save_config(opt, &(*CONFIG.lock().await)).await?;
        Ok("Finished".to_string())
    } else {
        Err(PCHStdError::EmptyInput)
    }
}

pub async fn remove(opt: &Command, _data: &Option<Vec<String>>) -> Result<String, PCHStdError> {
    if let Some(data) = _data {
        for data in data.iter() {
            let index = match CONFIG
                .lock()
                .await
                .file_names
                .iter()
                .position(|x| x == data)
            {
                Some(index) => index,
                None => {
                    warn!("Remove failed:", data, "not found");
                    continue;
                }
            };
            CONFIG.lock().await.file_names.remove(index);
            ok!("Removed:", data);
        }
        save_config(opt, &(*CONFIG.lock().await)).await?;
        Ok("Finished".to_string())
    } else {
        Err(PCHStdError::EmptyInput)
    }
}

pub async fn save_config(opt: &Command, config: &Config) -> Result<String, PCHStdError> {
    let config_str = serde_json::to_string_pretty(&config)?;
    let config_url = match opt {
        Command::Add { config, .. } => config,
        Command::Remove { config, .. } => config,
        Command::Rename { config, .. } => config,
    };
    save_file(config_url, &config_str).await?;
    Ok("Finished".to_string())
}
