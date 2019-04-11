use genere::Generator;

fn main() {
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]"],
   "job[hero]": ["wizard/witch"],
   "main[hero]": ["{hero}. He/She is a {job}."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
