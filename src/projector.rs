use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            projector: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Projector {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut paths = vec![];
        let mut curr = Some(self.pwd.as_path());

        while let Some(path) = curr {
            paths.push(path);
            curr = path.parent();
        }

        let mut out = HashMap::new();

        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.into_iter());
            }
        }

        out
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.pwd.as_path());
        let mut out = None;

        while let Some(path) = curr {
            if let Some(dir) = self.data.projector.get(path) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }

            curr = path.parent();
        }

        out
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data
            .projector
            .get_mut(&self.pwd)
            .map(|map| map.remove(key));
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = self.config.parent() {
            if !std::fs::metadata(&path).is_ok() {
                std::fs::create_dir_all(path)?;
            }
        }

        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;

        Ok(())
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or("{\"projector\":{}}".to_string());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data::default());

            return Self { config, pwd, data };
        }

        Self {
            config,
            pwd,
            data: Data::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Data, Projector};
    use collection_macros::hashmap;
    use std::{collections::HashMap, path::PathBuf};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        hashmap! {
            PathBuf::from("/") => hashmap! {
                "foo".into() => "bar".into(),
                "fem".into() => "is_great".into(),
            },
            PathBuf::from("/foo") => hashmap! {
                "foo".into() => "bar2".into(),
            },
            PathBuf::from("/foo/bar") => hashmap! {
                "foo".into() => "bar3".into(),
            },
        }
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        Projector {
            config: PathBuf::from(""),
            pwd,
            data: Data {
                projector: get_data(),
            },
        }
    }

    #[test]
    fn get_value() {
        let projector = get_projector(PathBuf::from("/foo/bar"));

        assert_eq!(projector.get_value("foo"), Some(&"bar3".to_string()));
        assert_eq!(projector.get_value("fem"), Some(&"is_great".to_string()));
    }

    #[test]
    fn set_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));
        projector.set_value("foo".to_string(), "bar4".to_string());
        projector.set_value("fem".to_string(), "is_better".to_string());

        assert_eq!(projector.get_value("foo"), Some(&"bar4".to_string()));
        assert_eq!(projector.get_value("fem"), Some(&"is_better".to_string()));
    }

    #[test]
    fn remove_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));
        projector.remove_value("foo");
        projector.remove_value("fem");

        assert_eq!(projector.get_value("foo"), Some(&"bar2".to_string()));
        assert_eq!(projector.get_value("fem"), Some(&"is_great".to_string()));
    }
}
