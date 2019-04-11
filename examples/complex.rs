use genere::Generator;

fn main() {
    let json = r#"
{
    "hero": ["John[m]", "Olivia[f]", "Gail[n]"],
    "job[hero]": ["sorci·er·ère", "guerri·er·ère"],
    "arme": ["hache[f]", "épée[f]", "gourdin[m]"],
    "fullarme[arme]": ["un·e {arme} tranchant·e", "un·e {arme} imposant·e", "un·e gros·se {arme}"],
    "main[hero]": ["Il/Elle/Iel s'appelle {hero}. C'est un·e {job}. Il/Elle/Iel a {fullarme}"]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
