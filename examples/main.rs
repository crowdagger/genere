use genere::Generator;

fn main() {
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]", "Jon[n]"],
   "job[hero]": ["sorci·er·ère"],
   "main[hero]": ["{hero}. Il/Elle/Iel est un·e {job}."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
