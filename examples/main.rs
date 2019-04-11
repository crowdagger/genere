use genere::Generator;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;


fn main() {
    let json = r#"
{
   "hero[main]": ["John[m]", "Joan[f]"],
   "job[hero]": ["wizard/witch"],
   "main[hero]": ["{hero}. He/She is a {job}."]
}"#;
        
    let mut gen = Generator::new();
//    gen.add("hero", &["John[m]", "Joan[f]"]).unwrap();
//    gen.add("job[hero]", &["wizard/witch"]).unwrap();
    //    gen.add("main[hero]", &["{hero}. He/She is a {job}."]).unwrap();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
