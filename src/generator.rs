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
enum Gender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug)]
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
    replacements: Vec<Replacement>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            replacements: vec![],
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
      
        println!("test");
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
        println!("{:?}", replacement);
        
        
        self.replacements.push(replacement);
        Ok(())
    }

    /// Instantiate a replacement symbol
    pub fn instantiate(&self, symbol: &str) -> Result<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(\S*)\}").unwrap();
        }
        
        let mut replaced: HashMap<&str, Replaced> = HashMap::new();

        let mut rng = thread_rng();
        for r in &self.replacements {
            // Pick a random variant 
            let s:&str = if let Some(s) = r.content.choose(&mut rng) {
                s
            } else {
                ""
            };

            // Replace {symbols} with replacements
            let result = RE.replace_all(s, |caps: &Captures| {
                match replaced.get(&caps[1]) {
                    Some(replaced) => replaced.content.clone(),
                    None => String::new(),
                }
            });
            
            replaced.insert(&r.symbol,
                            Replaced {
                                gender: Gender::Neutral,
                                content: result.to_string()});
        }

        match replaced.get(&symbol) {
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
