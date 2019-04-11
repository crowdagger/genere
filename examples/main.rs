use genderly::Generator;

fn main() {
    let mut gen = Generator::new();
    gen.add("Test[plop]", &["coin", "bar"]);
    gen.add("Foo", &["Male/female"]);
    println!("{}", gen.instantiate("Foo").unwrap());
}
