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


## More information

Genere is inspired by [Tracery](http://tracery.io/), but seeks to allow easy generation
of sentences that are grammaticaly gender accurate.

## TODO

* Symbols must be declared in the order they will be used.
* Support for JSON.
* Support for "content·e" variants.

