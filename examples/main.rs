use genere::Generator;

fn main() {
    let s = Generator::pre_process(r"foo\{bar\}\[baz\]".to_string());
    println!("preprocess done: {}", s);

    let s = Generator::post_process(s);
    println!("postprocess done: {}", s);
    
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]", "Jon[n]"],
   "job": ["sorci·er·ère[hero]"],
   "main": ["{hero}. Il/Elle[hero] est un·e[hero] {job}."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
