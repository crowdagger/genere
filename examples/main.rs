use genere::Generator;

fn main() {
    let mut gen = Generator::new();
    gen.add("hero", &["John[m]", "Joan[f]"]).unwrap();
    gen.add("job[hero]", &["wizard/witch"]).unwrap();
    gen.add("main[hero]", &["{hero}. He/She is a {job}."]).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
