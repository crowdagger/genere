use genere::Generator;

fn main() {
    let json = r#"
{
    "hero": ["John[m]", "Olivia[f]", "Gail[n]"],
    "job[hero]": ["sorci·er·ère", "guerri·er·ère"],
    "arme": ["hache[f]", "épée[f]", "gourdin[m]"],
    "adjectif[arme]": ["tranchant·e", "imposant·e", "étincelant·e"],
    "main[hero]": ["Il/Elle/Iel s'appelle {hero}. C'est un·e {job}. Il/Elle/Iel a un·e[arme] {arme} {adjectif}"]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
