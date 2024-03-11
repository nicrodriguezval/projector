use crate::opts::Opts;
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,
    pub pwd: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Config {
            operation,
            config,
            pwd,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Add((String, String)),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;

        if value.is_empty() {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        let length = value.len();

        if term == "add" {
            if length != 3 {
                return Err(anyhow!(
                    "operation add expects 2 arguments but got {}",
                    length - 1
                ));
            }

            return Ok(Operation::Add((value[1].clone(), value[2].clone())));
        }

        if term == "rm" {
            if length != 2 {
                return Err(anyhow!(
                    "operation remove expects 1 argument but got {}",
                    length - 1
                ));
            }

            return Ok(Operation::Remove(value[1].clone()));
        }

        if length > 1 {
            return Err(anyhow!(
                "operation print expects 0 or 1 argument but got {}",
                length
            ));
        }

        let arg = value.pop().expect("to exist");
        Ok(Operation::Print(Some(arg)))
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(config) = config {
        return Ok(config);
    }

    let loc = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").expect("unable to get home directory");
        format!("{}/.config", home)
    });
    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    Ok(loc)
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    Ok(std::env::current_dir().context("unable to get current directory")?)
}
