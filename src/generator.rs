// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::errors::Result;

use std::collections::HashMap;

use regex::{Regex, Captures};
use lazy_static::lazy_static;
use rand::prelude::*;
use error_chain::bail;

#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug, Clone)]
struct Replaced {
    pub content: String,
    pub gender: Gender,
}

#[derive(Debug)]
struct Replacement {
    pub gender_dependency: Option<String>,
    pub content: Vec<String>,
}

pub struct Generator {
    replaced: HashMap<String, Replaced>,
    replacements: HashMap<String, Replacement>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            replacements: HashMap::new(),
            replaced: HashMap::new(),
        }
    }

    /// Adds a replacement grammar using JSON format.
    pub fn add_json(&mut self, json: &str) -> Result<()> {
        let map: HashMap<String, Vec<String>> = serde_json::from_str(json)?;

        for (symbol, content) in map {
            self.add_move(symbol, content)?;
        }
        Ok(())
    }

    /// Adds a replacement grammar that will replace given symbol by one of those elements.
    pub fn add(&mut self, symbol: &str, content: &[&str]) -> Result<()> {
        let symbol: String = symbol.into();

        let mut c: Vec<String> = Vec::with_capacity(content.len());
        for s in content {
            c.push(s.to_string());
        }
        self.add_move(symbol, c)
    }

    fn add_move(&mut self, symbol: String, content: Vec<String>) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)\[(\S*)\]").unwrap();
        }
        

      
        let cap = RE.captures(&symbol);
        let (symbol, replacement) = if let Some(cap) = cap {
            let symbol = cap[1].into();
            (symbol, Replacement {
                gender_dependency: Some(cap[2].into()),
                content: content,
            })
        } else {
            (symbol,
            Replacement {
                gender_dependency: None,
                content: content,
            })
        };
        
        
        self.replacements.insert(symbol, replacement);
        Ok(())
    }


    /// Sets a symbol to a gender
    pub fn set_gender(&mut self, symbol: &str, gender: Gender) {
        self.replaced.insert(symbol.into(), Replaced {
            gender: gender,
            content: String::new()
        });
    }


    fn instantiate_util(&self, symbol: &str, replaced: &mut HashMap<String, Replaced>,
                        rng: &mut ThreadRng) -> Result<String> {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(\S*)\}").unwrap();
            static ref RE_GENDER: Regex = Regex::new(r"[/.]").unwrap();
            static ref RE_SET_GENDER: Regex = Regex::new(r"\[(\S*)\]").unwrap();
            static ref RE_SLASHES: Regex = Regex::new(r"(\S*)/(\S*)").unwrap();
        }


        if let Some(r) = self.replacements.get(symbol) {
            let mut gender = Gender::Neutral;
            
            // Pick a random variant 
            let s:&str = if let Some(s) = r.content.choose(rng) {
                s
            } else {
                ""
            };

            // Set the gender of the symbol, if needed
            {
                let mut i = 0;
                for caps in RE_SET_GENDER.captures_iter(s) {
                    i += 1;
                    if i > 1 {
                        bail!("multiple genders for symbol '{}' in expression '{}'",
                              symbol,
                              s);
                    }
                    match &caps[1] {
                        "m" | "M" => gender = Gender::Male,
                        "f" | "F" => gender = Gender::Female,
                        "n" | "N" => gender = Gender::Neutral,
                        _ => bail!("invalid gender {} for symbol '{}' in expression '{}'",
                                   &caps[1],
                                   symbol,
                                   s),
                    }
                    println!("set gender for {} : {:?}", symbol, gender);
                }
            }
            
            let s = RE_SET_GENDER.replace_all(&s, "");
            
            // Replace {symbols} with replacements
            let result = RE.replace_all(s.as_ref(), |caps: &Captures| {
                self.instantiate_util(&caps[1], replaced, rng).unwrap()
            });
            

            // Gender adaptation, if needed
            // Find the gender to replace
            let gender_adapt = if let Some(key) = &r.gender_dependency {
                if !replaced.contains_key(key.as_str()) {
                    self.instantiate_util(key, replaced, rng)?;
                }
                match replaced.get(key.as_str()) {
                    Some(replaced) => replaced.gender,
                    None => bail!("Symbol {} needs a gender to be specified by {} but it doesn't specify one",
                                  symbol,
                                  key),
                }
            } else {
                Gender::Neutral
            };
            let result = RE_SLASHES.replace_all(&result, |caps: &Captures| {
                match gender_adapt {
                    Gender::Male => format!("{}", &caps[1]),
                    Gender::Female => format!("{}", &caps[2]),
                    Gender::Neutral => format!("{} / {}", &caps[1],&caps[2])
                }
            });

            replaced.insert(symbol.to_string(),
                            Replaced {
                                gender: gender,
                                content: result.to_string()});
        } else {
            bail!("could not find symbol {} in generator", symbol);
        }

        match replaced.get(symbol) {
            Some(replaced) => Ok(replaced.content.clone()),
            None => bail!("could not find symbol {} in generator", symbol)
        }

    }

    /// Instantiate a replacement symbol
    pub fn instantiate(&self, symbol: &str) -> Result<String> {
        let mut replaced = self.replaced.clone();

        let mut rng = thread_rng();

        self.instantiate_util(symbol, &mut replaced, &mut rng)
    }
}

#[test]
fn add_1() {
    let mut gen = Generator::new();
    gen.add("Test", &["foo", "bar"]).unwrap();
}

#[test]
fn add_json() {
    let mut gen = Generator::new();
    gen.add_json(r#"
{
   "Test": ["foo", "bar"]
}"#).unwrap();
}

#[test]
fn missing_symbol() {
    let mut gen = Generator::new();
    assert!(gen.instantiate("foo").is_err());
}

#[test]
fn replacement_1() {
    let mut gen = Generator::new();
    gen.add("foo", &["hello"]).unwrap();
    gen.add("bar", &["{foo} world"]).unwrap();
    assert_eq!(gen.instantiate("bar").unwrap(), String::from("hello world"));
}

#[test]
fn replacement_2() {
    let mut gen = Generator::new();
    gen.add("foo", &["hello"]).unwrap();
    gen.add("bar", &["world"]).unwrap();
    gen.add("baz", &["{foo} {bar}"]).unwrap();
    assert_eq!(gen.instantiate("baz").unwrap(), String::from("hello world"));
}

#[test]
fn gender_1() {
    let mut gen = Generator::new();
    gen.add("foo[plop]", &["He/She is happy"]).unwrap();
    gen.set_gender("plop", Gender::Male);
    assert_eq!(&gen.instantiate("foo").unwrap(), "He is happy");
    gen.set_gender("plop", Gender::Female);
    assert_eq!(&gen.instantiate("foo").unwrap(), "She is happy");
}


#[test]
fn gender_2() {
    let mut gen = Generator::new();
    gen.add("plop", &["Joe[m]"]).unwrap();
    gen.add("foo[plop]", &["He/She is happy"]).unwrap();
    assert_eq!(&gen.instantiate("foo").unwrap(), "He is happy");
}

