use anyhow::{anyhow, Result};

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
                term
            ));
        }

        let arg = value.pop().expect("to exist");
        Ok(Operation::Print(Some(arg)))
    }
}
