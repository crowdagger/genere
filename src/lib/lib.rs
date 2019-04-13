// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Genere is a library to generate (possibly randomized) text with options to match the (grammatical) gender
//! of various elements.
//!
//! # Example
//!
//! ```
//! use genere::Generator;
//! let json = r#"
//! {
//!    "hero": ["John[m]", "Joan[f]"],
//!    "job[hero]": ["wizard/witch"],
//!    "main[hero]": ["{hero}. He/She is a {job}."]
//! }"#;
//!
//! let mut gen = Generator::new();
//! gen.add_json(json).unwrap();;
//! let result = gen.instantiate("main").unwrap();
//! assert!(&result == "John. He is a wizard."
//!        || &result == "Joan. She is a witch.");
//! ```
//! 
//!
//! # Features
//!
//! ## Binary or Rust library
//!
//! It is possible to use Genere as a binary:
//!
//! ```bash
//! $ genere main < file.json
//! ```
//! will instantiate the `main` symbol in the `file.json` file.
//!
//! Genere is, however, primarily a [Rust](https://rust-lang.org) library, so it can be used in programs written in Rust: you only have to add
//!
//! ```toml
//! genere = "0.1"
//! ```
//!
//! In the `dependencies` section of your `Cargo.toml` file.
//!
//! ## Text generation
//!
//! Genere is inspired by [Tracery](http://tracery.io/) and thus has a similar syntax to allow
//! you to easily generate randonized text:
//!
//! ```
//! # use genere::Generator;
//! let json = r#"
//! {
//!     "name": ["John", "Johana", "Vivienne", "Eric"],
//!     "last_name": ["StrongArm", "Slayer", "The Red"],
//!     "class": ["mage", "warrior", "thief", "rogue", "barbarian"],
//!     "race": ["human", "dwarvish", "elvish", "vampire"],
//!     "text": ["{name} {last_name} is a {race} {class}.",
//! 	     "Meet {name} {last_name}, A proud {class}!"]
//! }
//! "#;
//!
//! # let mut gen = Generator::new();
//! # gen.add_json(json).unwrap();;
//! # let result = gen.instantiate("text").unwrap();
//! # println!("{}", result);
//! ```
//!
//! might display "Johana  Slayer is a vampire warrior."
//!
//! Basically, you define a list of symbols which will be replaced (randomly) by one version
//! of the string in the corresponding array when you "call" them using the `{symbol`} syntax.
//!
//! Not that once a symbol has been "instantiated", ils value is fixed. So if you had:
//!
//! ```json
//! "text": ["Meet {name} {last_name}. {name} is a proud {class}."]
//! ```
//!
//! it is guarenteed that both replacements for `{name}` will be identical.
//!
//! If you want to get a (possibly) different instantiation, you need to use `{{symbol}}`:
//!
//! ```json
//! "text": ["Meet {name} {last_name}. {name} is a proud {class}. There is also {{name}}, a {{class}}."]
//! ```
//!
//! ## Gender adaptation
//!
//! Genere seeks to allow easy generation of sentences that are grammaticaly gender accurate:
//!
//! ```
//! # use genere::Generator;
//! let json = r#"
//! {
//!     "name": ["John[m]", "Johana[f]", "Vivienne[f]", "Eric[m]"],
//!     "class": ["mage", "warrior", "thief", "rogue", "barbarian"],
//!     "text[name]": ["Meet {name}. He/She is a proud {class}!"]
//! }
//! "#;
//!
//! # let mut gen = Generator::new();
//! # gen.add_json(json).unwrap();;
//! # let result = gen.instantiate("text").unwrap();
//! # println!("{}", result);
//! ```
//!
//! will make sure to display "He" or She" according to the gender specified in the symbol `name`.
//!
//! You can set a gender to these values using the `[m]`, `[f]` or `[n]`. Similarly, you can
//! tell genere that a symbol depends on another's symbol gender by using `[symbol]` in the symbol name. E.g., `text[main]` means that the gender in `main`'s replacement strings will be determined by `name`'s gender.

//! It is also possible to specify a neutral gender, by using `[n]` in the definition and by
//! adding a `/` in the replacement string (e.g. `He/She/They`). If it isn't specified in the
//! replacement string, both male and female version will be outputted (e.g. `He/She` instead of `Them`).
//!
//! Sometimes a sentence might use various gendered elements and not just depend on only one symbol's gender.
//! For each gender variation, it is possible to specify a "dependency":
//!
//! ```json
//! "text[hero]": ["He/She is called {hero}. His/Her son/daughter[child] is named {child}."]
//! ```
//!
//! Here, the gender of `hero` will be used to determine between `He/She` and `His/Her`, but
//! the gender of `child` will be used to pick between `son/daughter`.
//!
//! ### Spaces in gender adaptation
//!
//! When you use this gender syntax, the '/' will only consider the word before and the word
//! after, not allowing to have spaces in your expressions. If you want to insert a space in a
//! gender adaptation expression, you must escape it with `~`, e.g.: "du/de~ la"
//!
//! ## Additional gender syntax
//!
//! It is also possible to use the "median point" syntax used e.g. in french: "C'est un·e sorci·er·ère." is equivalent to "C'est un/une sorcier/sorcière".
//!
//! ## Escaping
//!
//! If you want to use the '[', ']', '{', '}', '/' and '·' characters in your text, you can use
//! the escape character '\~'. E.g., "\~{foo}" will display "{foo}" instead of trying to find the symbol `foo` and replace it with its content. You can also use "~~" if you want to display the tilde symbol.
//!
//! ## License
//!
//! Genere is published under the Mozilla Public License, version 2.0. For more information, see the [License](LICENSE).
//!
//! ## ChangeLog
//!
//! See [ChangeLog](ChangeLog.md).


mod errors;
mod generator;

pub use generator::Generator;
pub use generator::Gender;
pub use errors::Result;

