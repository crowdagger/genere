use genere::Generator;

fn main() {
    let json = r#"
{
    "hero": ["John[m]", "Olivia[f]", "Gail[n]", "Tom[m]", "Judi[f]"],
    "job[hero]": ["sorci·er·ère", "guerri·er·ère", "voleu·r·se", "barbare", "archer/archère"],
    "arme": ["hache[f]", "épée[f]", "gourdin[m]", "arc[m]", "masse[f]"],
    "adjectif[arme]": ["tranchant·e", "imposant·e", "étincelant·e", "rouillé·e", "brutal·e"],
    "description": ["{hero}, un·e[hero] {job} avec un·e[arme] {arme} {adjectif}"],
    "main[hero]": ["Il/Elle/Iel s'appelle {hero}. {hero} est un·e {job}. Il/Elle/Iel a un·e[arme] {arme}. Ce·tte[arme]  {arme} est {adjectif}. Avec lui/elle se trouve {{description}} et {{description}}. {hero} les aime bien, c'est son crew."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
    println!("{}", gen.instantiate("main").unwrap());
}
