use genere::Generator;

fn main() {
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]", "Jon[n]"],
   "job[hero]": ["wizard/witch/sorcerer"],
   "main[hero]": ["{hero}. He/She/They is a {job}."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
