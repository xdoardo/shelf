use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use clap::{Parser, Subcommand};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

fn get_data_path() -> String {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "shelf") {
        proj_dirs.config_dir().clone().to_string_lossy().to_string()
    } else {
        String::from_str("~/.shelf").unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    id: String,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(Vec<Item>);

impl Default for Data {
    fn default() -> Self {
        Data(vec![])
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// The directory to store the file map in.
    #[arg(long, default_value_t=get_data_path())]
    pub dir: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Remove a previously added mark
    Remove { id: String },

    /// Open a previously added mark
    Open { id: String },

    /// Add a new mark with <id> to <file>
    Add { id: String, file: PathBuf },

    /// List all recorded marks
    List,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub home: PathBuf,
}

fn write_data(data: Data, conf: AppConfig) -> Result<(), anyhow::Error> {
    let mut path = conf.home.clone();

    fs::create_dir_all(path.clone())?;

    path.push("files");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let to_write = ron::to_string(&data)?;

    Ok(write!(file, "{}", to_write)?)
}

fn read_data(conf: AppConfig) -> anyhow::Result<Data> {
    let mut path = conf.home.clone();
    path.push("files");

    if !path.exists() {
        write_data(Data::default(), conf.clone())?;
    }

    return Ok(ron::from_str(&fs::read_to_string(path)?)?);
}

pub fn remove(id: String, conf: AppConfig) -> anyhow::Result<()> {
    let mut data: Data = read_data(conf.clone())?;
    for (i, f) in data.0.clone().into_iter().enumerate() {
        if id == f.id {
            data.0.remove(i);
            break;
        }
    }

    write_data(data, conf)
}

pub fn open(id: String, conf: AppConfig) -> anyhow::Result<()> {
    let data = read_data(conf)?;

    for f in data.0 {
        if id == f.id {
            return Ok(open::that_detached(f.file)?);
        }
    }
    anyhow::bail!("{id} not found!")
}

pub fn add(id: String, file: PathBuf, conf: AppConfig) -> anyhow::Result<()> {
    let mut data = read_data(conf.clone())?;
    let mut to_append = true;
    for (i, f) in data.0.clone().into_iter().enumerate() {
        if f.id == id {
            let el = data.0.get_mut(i).unwrap();
            el.file = file.clone().into_os_string().to_string_lossy().to_string();
            to_append = false;
            break;
        }
    }

    if to_append {
        if let Some(file) = file.to_str() {
            data.0.push(Item {
                id,
                file: file.to_string(),
            })
        } else {
            anyhow::bail!("file {} contains non-unicode characters!", file.display())
        }
    }

    write_data(data, conf)
}

pub fn list(conf: AppConfig) -> anyhow::Result<()> {
    let data = read_data(conf)?;

    for f in data.0 {
        println!("{} => {}", f.id, f.file);
    }
    Ok(())
}
