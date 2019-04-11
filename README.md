# genere

Genere is a library to generate (possibly randomized) text with options to match the (grammatical) gender
of various elements.

## Example

```rust
use genere::Generator;
let mut gen = Generator::new();
gen.add("hero", &["John[m]", "Joan[f]"]).unwrap();
gen.add("job[hero]", &["wizard/witch"]).unwrap();
gen.add("main[hero]", &["{hero}. He/She is a {job}."]).unwrap();
let result = gen.instantiate("main").unwrap();
assert!(&result == "John. He is a wizard."
       || &result == "Joan. She is a witch.");
```


## Same example, using JSON

```rust
use genere::Generator;
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]"],
   "job[hero]": ["wizard/witch"],
   "main[hero]": ["{hero}. He/She is a {job}."]
}"#;

let mut gen = Generator::new();
gen.add_json(json).unwrap();;
let result = gen.instantiate("main").unwrap();
assert!(&result == "John. He is a wizard."
       || &result == "Joan. She is a witch.");
```


## More information

Genere is inspired by [Tracery](http://tracery.io/), but seeks to allow easy generation
of sentences that are grammaticaly gender accurate.

Basically, you define a list of symbols which will be replaced (randomly) by one version
of the string in the corresponding array.

You can set a gender to these values using the `[m]`, `[f]` or `[n]`. Similarly, you can
tell genere that a symbol depends on another's symbol gender by using `[symbol]` in the symbol name.

E.g., "main[hero]" means that the gender in `main`'s replacement strings will be determined
by `hero`'s gender.

`main`'s replacement strings can then use Male/Female syntax (e.g. `He/She`) and the appropriate
version will be picked up depending on `hero`'s gender.

It is also possible to specify a neutral gender, by using `[n]` in the definition and by
adding a `/` in the replacement string (e.g. `He/She/They`). If it isn't specify in the
replacement string, both male and female version will be outputted (e.g. `He/She` instead of `Them`).


## TODO

* Support for "content·e" variants.

