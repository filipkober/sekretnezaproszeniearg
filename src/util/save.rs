pub mod save {
    use std::{io::{self, Write}, fs::{File, self}, fmt::Debug};

    use json::{object, JsonValue, array};

    pub struct Level {
        pub name: String,
        pub time: f32,
        pub completed: bool
    }
    pub struct Savefile {
        pub entropy: String,
        pub levels: Vec<Level>
    }

    impl Clone for Level {
        fn clone(&self) -> Self {
            Level {
                name: self.name.clone(),
                time: self.time,
                completed: self.completed
            }
        }
    }

    impl Into<JsonValue> for Level {
        fn into(self) -> JsonValue {
            object! {
                name: self.name,
                time: self.time,
                completed: self.completed
            }
        }
    }

    impl From<JsonValue> for Level {
        fn from(value: JsonValue) -> Self {
            if !value.is_object() {
                return Level {
                    name: "".to_string(),
                    time: 0f32,
                    completed: false,
                };
            }
            let mut parsed = Level {
                name: "".to_string(),
                time: 0f32,
                completed: false
            };
            for (key, v) in value.entries() {
                match key {
                    "name" => {
                        if let Some(name) = v.as_str() {
                            parsed.name = name.to_string();
                        }
                    },
                    "time" => {
                        if let Some(time) = v.as_f32() {
                            parsed.time = time;
                        }
                    },
                    "completed" => {
                        if let Some(completed) = v.as_bool() {
                            parsed.completed = completed;
                        }
                    },
                    _ => {}
                };
            };
            return parsed;
        }
    }

    impl From<JsonValue> for Savefile {
        fn from(value: JsonValue) -> Self {
            if !value.is_object() {
                return Savefile {
                    entropy: "".to_string(),
                    levels: vec![]
                };
            }
            let mut parsed = Savefile {
                entropy: "".to_string(),
                levels: vec![]
            };
            for (key, v) in value.entries() {
                match key {
                    "entropy" => {
                        if let Some(entropy) = v.as_str() {
                            parsed.entropy = entropy.to_string();
                        }
                    },
                    "levels" => {
                            for level in v.members() {
                                parsed.levels.push(Level::from(level.clone()));
                            }
                    },
                    _ => {}
                };
            };
            return parsed;
        }
    }

    impl Debug for Level {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Level")
                .field("name", &self.name)
                .field("time", &self.time)
                .field("completed", &self.completed)
                .finish()
        }
    }

    impl Debug for Savefile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Savefile")
                .field("entropy", &self.entropy)
                .field("levels", &self.levels)
                .finish()
        }
    }

    pub fn empty_savefile(entropy: String) -> Savefile {
        Savefile {
            entropy: entropy,
            levels: vec![
                Level {
                    time: 0f32,
                    completed: false,
                    name: "80808".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "HAZARD DUTY PAY!".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Krystle".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Rainbow Six".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Hollywood Baby".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Western Union".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "toothless".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "God Loves You".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Known For It".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Drake Era".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Out by 16, Dead on the Scene".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "The Fear".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Tantor".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "DEATHCAMP".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Burfict!".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "I Cannot Fucking Wait Til Morrissey Dies".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Free The Frail".to_string()
                },
                Level {
                    time: 0f32,
                    completed: false,
                    name: "Today".to_string()
                }
            ]
        }
    }

    pub fn save_settings(entropy: String, levels: Vec<Level>) -> Result<(), io::Error> {
        let levels_vec: Vec<JsonValue> = levels.into_iter().map(|l| {l.into()}).collect();
        let json_data = json::stringify(object!{
            entropy: entropy,
            levels: levels_vec
        });
        let mut file = File::create("data.json")?;
        file.write_all(json_data.as_bytes())
    }
    pub fn read_settings() -> Result<Savefile, io::Error> {
        let mut file_content = fs::read_to_string("data.json")?;
        let file_json = json::parse(&file_content).unwrap();
        Ok(file_json.into())
    }
    pub fn settings_exist() -> bool {
        fs::metadata("data.json").is_ok()
    }
}