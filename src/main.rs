use atom::Atom;

mod atom;

fn main() {
    let atom = Atom::new_with_symbol("He");
    println!("Atom Mass: {}", atom.get_mass());
}