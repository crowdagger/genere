// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::errors::Result;

use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

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

    /// Preprocess a string to replaced escaped characters that characters that won't
    /// interfere with genere's regexes.
    fn pre_process(s: String) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"~(.)").unwrap();
        }

        if RE.is_match(&s) {
            let new_s = RE.replace_all(&s, |caps: &Captures| {
                match &caps[1] {
                    " " => Cow::Borrowed(r"~<space>"),
                    r"~" => Cow::Borrowed(r"~<tilde>"),
                    r"[" => Cow::Borrowed(r"~<leftsquare>"),
                    r"]" => Cow::Borrowed(r"~<rightsquare>"),
                    r"{" => Cow::Borrowed(r"~<leftcurly>"),
                    r"}" => Cow::Borrowed(r"~<rightcurly>"),
                    r"/" => Cow::Borrowed(r"~<slash>"),
                    r"·" => Cow::Borrowed(r"~<median>"),
                    n => Cow::Owned(format!("{}", n)),
                }
            });
            new_s.into_owned()
        } else {
            s
        }
    }

    /// Prost-process a string to replace escape characters with expected ones
    fn post_process(s: String) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"~<(\w+)>").unwrap();
        }

        if RE.is_match(&s) {
            let new_s = RE.replace_all(&s, |caps: &Captures| {
                match &caps[1] {
                    "space" => " ",
                    "tilde" => r"~",
                    "leftsquare" => r"[",
                    "rightsquare" => r"]",
                    "leftcurly" => r"{",
                    "rightcurly" => r"}",
                    "slash" => "/",
                    "median" => "·",
                    _ => unreachable!(),                        
                }
            });
            new_s.into_owned()
        } else {
            s
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

    pub fn add_move(&mut self, mut symbol: String, mut content: Vec<String>) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)\[(\w*)\]").unwrap();
        }

        symbol = Self::pre_process(symbol);
        for i in 0..content.len() {
            let c = std::mem::replace(&mut content[i], String::new());
            content[i] = Self::pre_process(c);
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

    fn get_gender<R:Rng>(&self, symbol: &str, replaced: &mut HashMap<String, Replaced>,
                   rng: &mut R,
                   stack: &mut HashSet<String>) -> Result<Gender> { 
        if !replaced.contains_key(symbol) {
            self.instantiate_util(symbol, replaced, rng, stack)?;
        }
        match replaced.get(symbol) {
            Some(replaced) => Ok(replaced.gender),
            None => bail!("Some symbol needs a gender to be specified by {} but it doesn't specify one",
                                  symbol),
        }
    }



    /// "forget" all state and instantiate a symbol
    fn reinstantiate<R: Rng>(&self, symbol: &str, rng: &mut R) -> Result<String> {
        let mut replaced = self.replaced.clone();
        let mut stack = HashSet::new();

        self.instantiate_util(symbol, &mut replaced, rng, &mut stack)
    }
    
    /// Used to recursively instantiate each element
    fn instantiate_util<R: Rng>(&self, symbol: &str, replaced: &mut HashMap<String, Replaced>,
                               rng: &mut R,
                               stack: &mut HashSet<String>) -> Result<String> {

        lazy_static! {
            static ref RE_REINSTANTIATE: Regex = Regex::new(r"\{\{(\w*)\}\}").unwrap();
            static ref RE_INSTANTIATE: Regex = Regex::new(r"\{(\w*)\}").unwrap();
            static ref RE_SET_GENDER: Regex = Regex::new(r"\[([mfn])\]").unwrap();
            static ref RE_SLASHES: Regex = Regex::new(r"([\w~<>]*)/([\w~<>]*)(?:/([\w~<>]*))?(?:\[(\w+)\])?").unwrap();
            static ref RE_DOTS: Regex = Regex::new(r"([\w~<>]+)·([\w~<>]*)(?:·([\w~<>]*))?(?:·([\w~<>]*))?(?:\[([\w~<>]+)\])?").unwrap();
        }

        // If symbol has already been instantiated, early return
        if let Some(r) = replaced.get(symbol) {
            return Ok(r.content.clone());
        }

        if stack.contains(symbol) {
            bail!("Can not instantiate, there is cyclic dependency: '{}' depends on itself!", symbol)
        }
        stack.insert(symbol.to_string());


        if let Some(r) = self.replacements.get(symbol) {
            let mut gender = Gender::Neutral;
            
            // Pick a random variant 
            let s:&str = if let Some(s) = r.content.choose(rng) {
                s
            } else {
                ""
            };

            // Set the gender of the symbol, if needed
            // If not [m] [f] or [n] it is a dependency, not a gender set
            {
                let mut i = 0;
                for caps in RE_SET_GENDER.captures_iter(s) {
                    i += 1;
                    if i > 1 {
                        bail!("Multiple genders for symbol '{}' in expression '{}'",
                              symbol,
                              s);
                    }
                    match &caps[1] {
                        "m" | "M" => gender = Gender::Male,
                        "f" | "F" => gender = Gender::Female,
                        "n" | "N" => gender = Gender::Neutral,
                        _ => unreachable!{},
                    }
                }
            }
            
            let s = RE_SET_GENDER.replace_all(&s, "");

            // Replace {{symbols}} with replacements, forgetting the environment
            let result = RE_REINSTANTIATE.replace_all(s.as_ref(), |caps: &Captures| {
                self.reinstantiate(&caps[1], rng).unwrap()
            });
            
            // Replace {symbols} with replacements
            let result = RE_INSTANTIATE.replace_all(result.as_ref(), |caps: &Captures| {
                self.instantiate_util(&caps[1], replaced, rng, stack).unwrap()
            });
            

            // Gender adaptation, if needed
            // Find the gender to replace
            let dependency = r.gender_dependency.as_ref();
            let gender_adapt = if let Some(key) = dependency {
                self.get_gender(key, replaced, rng, stack)?
            } else {
                Gender::Neutral
            };

            // Replacement of the form "content·e" (used in french)
            let result = RE_DOTS.replace_all(&result, |caps: &Captures| {
                let mut len = 3;
                if caps.get(3).is_some() {
                    len += 1;
                }
                if caps.get(4).is_some() {
                    len += 1;
                }
                let gender = if caps.get(5).is_some() {
                    self.get_gender(&caps[5], replaced, rng, stack).unwrap()
                } else {
                    gender_adapt
                };
                match gender {
                    Gender::Male => match len {
                        3 => format!("{}", &caps[1]),
                        4 => format!("{}{}", &caps[1], &caps[2]),
                        5 => format!("{}{}{}", &caps[1], &caps[2], &caps[4]),
                        _ => unreachable!{}
                    }
                    Gender::Female => match len {
                        3 => format!("{}{}", &caps[1], &caps[2]),
                        4 => format!("{}{}", &caps[1], &caps[3]),
                        5 => format!("{}{}{}", &caps[1], &caps[3], &caps[4]),
                        _ => unreachable!{}
                    }
                    Gender::Neutral => match len {
                        3 => format!("{rad}/{rad}{f}",
                                     rad = &caps[1],
                                     f = &caps[2]),
                        4 => format!("{rad}{m}/{rad}{f}",
                                     rad = &caps[1],
                                     m = &caps[2],
                                     f = &caps[3]),
                        5 => format!("{rad}{m}{s}/{rad}{f}{s}",
                                     rad = &caps[1],
                                     m = &caps[2],
                                     f = &caps[3],
                                     s = &caps[4]),
                        _ => unreachable!{}
                    }
                }
            });

            // Replacement of the form Male/Female[/Neutral]
            let result = RE_SLASHES.replace_all(&result, |caps: &Captures| {
                let gender = if caps.get(4).is_some() {
                    self.get_gender(&caps[4], replaced, rng, stack).unwrap()
                } else {
                    gender_adapt
                };
                
                match gender {
                    Gender::Male => format!("{}", &caps[1]),
                    Gender::Female => format!("{}", &caps[2]),
                      Gender::Neutral => if caps.get(3).is_some() {
                          format!("{}", &caps[3])
                      } else {
                          format!("{}/{}", &caps[1],&caps[2])
                      }
                  }
            });


            replaced.insert(symbol.to_string(),
                            Replaced {
                                gender: gender,
                                content: result.to_string()});
        } else {
            bail!("could not find symbol {} in generator", symbol);
        }

        stack.remove(symbol);
        
        match replaced.get(symbol) {
            Some(replaced) => Ok(replaced.content.clone()),
            None => unreachable!{},
        }

    }

    /// Instantiate a replacement symbol
    pub fn instantiate(&self, symbol: &str) -> Result<String> {
        let mut replaced = self.replaced.clone();
        let mut rng = thread_rng();
        let mut set = HashSet::new();

        let final_s = self.instantiate_util(symbol, &mut replaced, &mut rng, &mut set)?;
        Ok(Self::post_process(final_s))
    }

    /// Instantiate a replacement symbol using a fixed seed.
    ///
    /// Useful if you want deterministic behaviour. 
    pub fn instantiate_from_seed(&self, symbol: &str, seed: u64) -> Result<String> {
        let mut replaced = self.replaced.clone();
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut set = HashSet::new();

        let final_s = self.instantiate_util(symbol, &mut replaced, &mut rng, &mut set)?;
        Ok(Self::post_process(final_s))
    }
}




///////////////////////////////////////////////////////////////////////////////////////////
//                                    TESTS
///////////////////////////////////////////////////////////////////////////////////////////



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
    let gen = Generator::new();
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

#[test]
fn gender_3() {
    let mut gen = Generator::new();
    gen.add("plop", &["Joe[m]"]).unwrap();
    gen.add("foo", &["He/She[plop] is happy"]).unwrap();
    assert_eq!(&gen.instantiate("foo").unwrap(), "He is happy");
}

#[test]
fn gender_4() {
    let mut gen = Generator::new();
    gen.add("arme", &["batte[f]"]).unwrap();
    gen.add("foo", &["Un homme au/à~ la[arme] {arme} facile"]).unwrap();
    assert_eq!(&gen.instantiate("foo").unwrap(), "Un homme à la batte facile");
}

#[test]
fn cyclic() {
    let mut gen = Generator::new();
    let json = r#"
{
   "a[b]": ["Foo"],
   "b[a]": ["Bar"]
}"#;
    gen.add_json(json).unwrap();
    assert!(gen.instantiate("a").is_err());
}

#[test]
fn unexisting() {
    let mut gen = Generator::new();
    let json = r#"
{
   "a[b]": ["Foo"],
   "b[c]": ["Bar"]
}"#;
    gen.add_json(json).unwrap();
    assert!(gen.instantiate("a").is_err());
}

#[test]
fn pre_process() {
    let s = Generator::pre_process(r"foobarbaz".to_string());
    assert_eq!(&s, "foobarbaz");

    let s = Generator::pre_process(r"~foobarbaz".to_string());
    assert_eq!(&s, "foobarbaz");

    let s = Generator::pre_process(r"~~foobarbaz".to_string());
    assert_eq!(&s, r"~<tilde>foobarbaz");

    let s = Generator::pre_process(r"foo~ bar~ baz".to_string());
    assert_eq!(&s, r"foo~<space>bar~<space>baz");


    let s = Generator::pre_process(r"~[foobarbaz~]".to_string());
    assert_eq!(&s, r"~<leftsquare>foobarbaz~<rightsquare>");
    
    let s = Generator::pre_process(r"~{foobarbaz~}".to_string());
    assert_eq!(&s, r"~<leftcurly>foobarbaz~<rightcurly>");

    let s = Generator::pre_process(r"foo/bar/baz".to_string());
    assert_eq!(&s, r"foo/bar/baz");
    
    let s = Generator::pre_process(r"foo~/bar~/baz".to_string());
    assert_eq!(&s, r"foo~<slash>bar~<slash>baz");

    let s = Generator::pre_process(r"foo~·bar~·baz".to_string());
    assert_eq!(&s, r"foo~<median>bar~<median>baz");
}

#[test]
fn post_process() {
    let s = String::from("No characters to replace here");
    let new_s = Generator::post_process(Generator::pre_process(s.clone()));
    assert_eq!(s, new_s);

    let s = String::from(r"~[Characters~] ~{to~} replace~ here~/and there~~");
    let new_s = Generator::post_process(Generator::pre_process(s));
    assert_eq!(&new_s, r"[Characters] {to} replace here/and there~");
}

#[test]
fn a_bit_all() {
    let json = r#"
{
   "object": ["~[lame~][f]"],
   "main": ["~{Vous~} avez un·e[object] {object}"]
}
"#;
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    let s = gen.instantiate("main").unwrap();
    assert_eq!(&s, r"{Vous} avez une [lame]");
}

#[test]
fn seed() {
    let json = r#"
{
    "hero": ["John[m]", "Olivia[f]", "Gail[n]", "Tom[m]", "Judi[f]"],
    "job[hero]": ["sorci·er·ère", "guerri·er·ère", "voleu·r·se", "barbare", "archer/archère"],
    "arme": ["hache[f]", "épée[f]", "gourdin[m]", "arc[m]", "masse[f]"],
    "adjectif[arme]": ["tranchant·e", "imposant·e", "étincelant·e", "rouillé·e", "brutal·e"],
    "description": ["{hero}, un·e[hero] {job} avec un·e[arme] {arme} {adjectif}"],
    "main[hero]": ["Il/Elle/Iel s'appelle {hero}. {hero} est un·e {job}. Il/Elle/Iel a un·e[arme] {arme}. Ce·tte[arme]  {arme} est {adjectif}. Avec lui/elle se trouve {{description}} et {{description}}. {hero} les aime bien, c'est son crew."]
}"#;
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    let r1 = gen.instantiate_from_seed("main", 42).unwrap();
    let r2 = gen.instantiate_from_seed("main", 42).unwrap();
    assert_eq!(r2, r1);
}
