mod molgraph;

fn main() {
    let smiles_str = "CC"; // Ethanol-like structure (C-C-O)
    let mol = molgraph::parse_smiles(smiles_str);

    println!("Parsed Molecule: {:#?}", mol);
    let generated_smiles = molgraph::to_smiles(&mol);
    println!("Generated SMILES: {}", generated_smiles);
}
