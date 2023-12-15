use std::{
    fmt::Debug,
    fs::{self, File},
    io::{self, Write},
};

use json::{object, JsonValue};

#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub time: f32,
    pub completed: bool,
    pub used_hints: usize,
}
#[derive(Debug)]
pub struct Savefile {
    pub entropy: String,
    pub levels: Vec<Level>,
}

impl Into<JsonValue> for Level {
    fn into(self) -> JsonValue {
        object! {
            name: self.name,
            time: self.time,
            completed: self.completed,
            used_hints: self.used_hints
        }
    }
}

impl From<JsonValue> for Level {
    fn from(value: JsonValue) -> Self {
        let mut parsed = Level {
            name: "".to_string(),
            time: 0f32,
            completed: false,
            used_hints: 0
        };
        if !value.is_object() {
            return parsed
        }
        for (key, v) in value.entries() {
            match key {
                "name" => {
                    if let Some(name) = v.as_str() {
                        parsed.name = name.to_string();
                    }
                }
                "time" => {
                    if let Some(time) = v.as_f32() {
                        parsed.time = time;
                    }
                }
                "completed" => {
                    if let Some(completed) = v.as_bool() {
                        parsed.completed = completed;
                    }
                }
                "used_hints" => {
                    if let Some(uhints) = v.as_usize() {
                        parsed.used_hints = uhints;
                    }
                }
                _ => {}
            };
        }
        return parsed;
    }
}

impl From<JsonValue> for Savefile {
    fn from(value: JsonValue) -> Self {
        if !value.is_object() {
            return Savefile {
                entropy: "".to_string(),
                levels: vec![],
            };
        }
        let mut parsed = Savefile {
            entropy: "".to_string(),
            levels: vec![],
        };
        for (key, v) in value.entries() {
            match key {
                "entropy" => {
                    if let Some(entropy) = v.as_str() {
                        parsed.entropy = entropy.to_string();
                    }
                }
                "levels" => {
                    for level in v.members() {
                        parsed.levels.push(Level::from(level.clone()));
                    }
                }
                _ => {}
            };
        }
        return parsed;
    }
}

pub fn empty_savefile(entropy: String) -> Savefile {
    Savefile {
        entropy: entropy,
        levels: vec![
            Level {
                time: 0f32,
                completed: false,
                name: "80808".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "HAZARD DUTY PAY!".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Krystle".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Rainbow Six".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Hollywood Baby".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Western Union".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "toothless".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "God Loves You".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Known For It".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Drake Era".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Out by 16, Dead on the Scene".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "The Fear".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Tantor".to_string(),
                used_hints: 0
            },
            Level {
                time: 0f32,
                completed: false,
                name: "DEATHCAMP".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Burfict!".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "The 27 Club".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Free The Frail".to_string(),
                used_hints: 0,
            },
            Level {
                time: 0f32,
                completed: false,
                name: "Today".to_string(),
                used_hints: 0,
            },
        ],
    }
}

pub fn save_settings(entropy: String, levels: Vec<Level>) -> Result<(), io::Error> {
    let levels_vec: Vec<JsonValue> = levels.into_iter().map(|l| l.into()).collect();
    let json_data = json::stringify(object! {
        entropy: entropy,
        levels: levels_vec
    });
    let mut file = File::create("data.json")?;
    file.write_all(json_data.as_bytes())
}

pub fn save_savefile(savefile: &Savefile) -> Result<(), io::Error> {
    save_settings(savefile.entropy.to_string(), savefile.levels.clone())
}

pub fn read_settings() -> Result<Savefile, io::Error> {
    let file_content = fs::read_to_string("data.json")?;
    let file_json = json::parse(&file_content).unwrap();
    Ok(file_json.into())
}
pub fn settings_exist() -> bool {
    fs::metadata("data.json").is_ok()
}
