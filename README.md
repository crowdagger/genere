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
