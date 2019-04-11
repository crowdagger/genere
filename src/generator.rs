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
    pub symbol: String,
    pub gender_dependency: Option<String>,
    pub content: Vec<String>,
}

pub struct Generator {
    replaced: HashMap<String, Replaced>,
    replacements: Vec<Replacement>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            replacements: vec![],
            replaced: HashMap::new(),
        }
    }

    /// Adds a replacement grammar that will replace given symbol by one of those elements.
    pub fn add(&mut self, symbol: &str, content: &[&str]) -> Result<()>{
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)\[(\S*)\]").unwrap();
        }
        
        let symbol: String = symbol.into();
        let mut c: Vec<String> = Vec::with_capacity(content.len());
        for s in content {
            c.push(s.to_string());
        }
      
        let cap = RE.captures(&symbol);
        let replacement = if let Some(cap) = cap {
            Replacement {
                symbol: cap[1].into(),
                gender_dependency: Some(cap[2].into()),
                content: c,
            }
        } else {
            Replacement {
                symbol: symbol,
                gender_dependency: None,
                content: c,
            }
        };
        
        
        self.replacements.push(replacement);
        Ok(())
    }

    /// Sets a symbol to a gender
    pub fn set_gender(&mut self, symbol: &str, gender: Gender) {
        self.replaced.insert(symbol.into(), Replaced {
            gender: gender,
            content: String::new()
        });
    }
        

    /// Instantiate a replacement symbol
    pub fn instantiate(&self, symbol: &str) -> Result<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(\S*)\}").unwrap();
            static ref RE_GENDER: Regex = Regex::new(r"[/.]").unwrap();
            static ref RE_SET_GENDER: Regex = Regex::new(r"\[(\S*)\]").unwrap();
            static ref RE_SLASHES: Regex = Regex::new(r"(\S*)/(\S*)").unwrap();
        }
        
        let mut replaced = self.replaced.clone();

        let mut rng = thread_rng();
        for r in &self.replacements {
            let mut gender = Gender::Neutral;
            
            // Pick a random variant 
            let s:&str = if let Some(s) = r.content.choose(&mut rng) {
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
                              r.symbol,
                              s);
                    }
                    match &caps[1] {
                        "m" | "M" => gender = Gender::Male,
                        "f" | "F" => gender = Gender::Female,
                        "n" | "N" => gender = Gender::Neutral,
                        _ => bail!("invalid gender {} for symbol '{}' in expression '{}'",
                                   &caps[1],
                                   r.symbol,
                                   s),
                    }
                }
            }
            
            let s = RE_SET_GENDER.replace_all(&s, "");
            
            // Replace {symbols} with replacements
            let result = RE.replace_all(s.as_ref(), |caps: &Captures| {
                match replaced.get(&caps[1]) {
                    Some(replaced) => replaced.content.clone(),
                    None => String::new(),
                }
            });
            

            // Gender adaptation, if needed
            // Find the gender to replace
            let gender_adapt = if let Some(key) = &r.gender_dependency {
                match replaced.get(key.as_str()) {
                    Some(replaced) => replaced.gender,
                    None => Gender::Neutral
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

            replaced.insert(r.symbol.clone(),
                            Replaced {
                                gender: gender,
                                content: result.to_string()});
        }

        match replaced.get(symbol) {
            Some(replaced) => Ok(replaced.content.clone()),
            None => bail!("could not find symbol {} in generator", symbol)
        }
    }
}

#[test]
fn add_1() {
    let mut gen = Generator::new();
    gen.add("Test", &["foo", "bar"]).unwrap();
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

