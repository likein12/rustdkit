pub enum AtomType {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Sodium,
    Magnesium,
    Aluminum,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,
    Argon,
    Potassium,
    Calcium,
    Scandium,
    Titanium,
    Vanadium,
    Chromium,
    Manganese,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,
    Krypton,
    Rubidium,
    Strontium,
    Yttrium,
    Zirconium,
    Niobium,
    Molybdenum,
    Technetium,
    Ruthenium,
    Rhodium,
    Palladium,
    Silver,
    Cadmium,
    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,
    Xenon,
    Cesium,
    Barium,
    Lanthanum,
    Cerium,
    Praseodymium,
    Neodymium,
    Promethium,
    Samarium,
    Europium,
    Gadolinium,
    Terbium,
    Dysprosium,
    Holmium,
    Erbium,
    Thulium,
    Ytterbium,
    Lutetium,
    Hafnium,
    Tantalum,
    Tungsten,
    Rhenium,
    Osmium,
    Iridium,
    Platinum,
    Gold,
    Mercury,
    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
    Radon,
    Francium,
    Radium,
    Actinium,
    Thorium,
    Protactinium,
    Uranium,
    Neptunium,
    Plutonium,
    Americium,
    Curium,
    Berkelium,
    Californium,
    Einsteinium,
    Fermium,
    Mendelevium,
    Nobelium,
    Lawrencium,
    Rutherfordium,
    Dubnium,
    Seaborgium,
    Bohrium,
    Hassium,
    Meitnerium,
    Darmstadtium,
    Roentgenium,
    Copernicium,
}

impl AtomType {
    fn get_mass(&self) -> f64 {
        match self {
            AtomType::Hydrogen => 1.008,
            AtomType::Helium => 4.0026,
            AtomType::Lithium => 6.94,
            AtomType::Beryllium => 9.0122,
            AtomType::Boron => 10.81,
            AtomType::Carbon => 12.011,
            AtomType::Nitrogen => 14.007,
            AtomType::Oxygen => 15.999,
            AtomType::Fluorine => 18.998,
            AtomType::Neon => 20.180,
            AtomType::Sodium => 22.990,
            AtomType::Magnesium => 24.305,
            AtomType::Aluminum => 26.982,
            AtomType::Silicon => 28.085,
            AtomType::Phosphorus => 30.974,
            AtomType::Sulfur => 32.06,
            AtomType::Chlorine => 35.45,
            AtomType::Argon => 39.948,
            AtomType::Potassium => 39.098,
            AtomType::Calcium => 40.078,
            AtomType::Scandium => 44.956,
            AtomType::Titanium => 47.867,
            AtomType::Vanadium => 50.942,
            AtomType::Chromium => 51.996,
            AtomType::Manganese => 54.938,
            AtomType::Iron => 55.845,
            AtomType::Cobalt => 58.933,
            AtomType::Nickel => 58.693,
            AtomType::Copper => 63.546,
            AtomType::Zinc => 65.38,
            AtomType::Gallium => 69.723,
            AtomType::Germanium => 72.63,
            AtomType::Arsenic => 74.922,
            AtomType::Selenium => 78.971,
            AtomType::Bromine => 79.904,
            AtomType::Krypton => 83.798,
            AtomType::Rubidium => 85.468,
            AtomType::Strontium => 87.62,
            AtomType::Yttrium => 88.906,
            AtomType::Zirconium => 91.224,
            AtomType::Niobium => 92.906,
            AtomType::Molybdenum => 95.95,
            AtomType::Technetium => 98.0,
            AtomType::Ruthenium => 101.07,
            AtomType::Rhodium => 102.91,
            AtomType::Palladium => 106.42,
            AtomType::Silver => 107.87,
            AtomType::Cadmium => 112.41,
            AtomType::Indium => 114.82,
            AtomType::Tin => 118.71,
            AtomType::Antimony => 121.76,
            AtomType::Tellurium => 127.6,
            AtomType::Iodine => 126.9,
            AtomType::Xenon => 131.29,
            AtomType::Cesium => 132.91,
            AtomType::Barium => 137.33,
            AtomType::Lanthanum => 138.91,
            AtomType::Cerium => 140.12,
            AtomType::Praseodymium => 140.91,
            AtomType::Neodymium => 144.24,
            AtomType::Promethium => 145.0,
            AtomType::Samarium => 150.36,
            AtomType::Europium => 151.96,
            AtomType::Gadolinium => 157.25,
            AtomType::Terbium => 158.93,
            AtomType::Dysprosium => 162.5,
            AtomType::Holmium => 164.93,
            AtomType::Erbium => 167.26,
            AtomType::Thulium => 168.93,
            AtomType::Ytterbium => 173.04,
            AtomType::Lutetium => 174.97,
            AtomType::Hafnium => 178.49,
            AtomType::Tantalum => 180.95,
            AtomType::Tungsten => 183.84,
            AtomType::Rhenium => 186.21,
            AtomType::Osmium => 190.23,
            AtomType::Iridium => 192.22,
            AtomType::Platinum => 195.08,
            AtomType::Gold => 196.97,
            AtomType::Mercury => 200.59,
            AtomType::Thallium => 204.38,
            AtomType::Lead => 207.2,
            AtomType::Bismuth => 208.98,
            AtomType::Polonium => 209.0,
            AtomType::Astatine => 210.0,
            AtomType::Radon => 222.0,
            AtomType::Francium => 223.0,
            AtomType::Radium => 226.0,
            AtomType::Actinium => 227.0,
            AtomType::Thorium => 232.04,
            AtomType::Protactinium => 231.04,
            AtomType::Uranium => 238.03,
            AtomType::Neptunium => 237.0,
            AtomType::Plutonium => 244.0,
            AtomType::Americium => 243.0,
            AtomType::Curium => 247.0,
            AtomType::Berkelium => 247.0,
            AtomType::Californium => 251.0,
            AtomType::Einsteinium => 252.0,
            AtomType::Fermium => 257.0,
            AtomType::Mendelevium => 258.0,
            AtomType::Nobelium => 259.0,
            AtomType::Lawrencium => 262.0,
            AtomType::Rutherfordium => 267.0,
            AtomType::Dubnium => 270.0,
            AtomType::Seaborgium => 271.0,
            AtomType::Bohrium => 270.0,
            AtomType::Hassium => 277.0,
            AtomType::Meitnerium => 276.0,
            AtomType::Darmstadtium => 281.0,
            AtomType::Roentgenium => 280.0,
            AtomType::Copernicium => 285.0
        }
    }
    
    fn get_elemental_symbol(&self) -> &str {
        match self {
            AtomType::Hydrogen => "H",
            AtomType::Helium => "He",
            AtomType::Lithium => "Li",
            AtomType::Beryllium => "Be",
            AtomType::Boron => "B",
            AtomType::Carbon => "C",
            AtomType::Nitrogen => "N",
            AtomType::Oxygen => "O",
            AtomType::Fluorine => "F",
            AtomType::Neon => "Ne",
            AtomType::Sodium => "Na",
            AtomType::Magnesium => "Mg",
            AtomType::Aluminum => "Al",
            AtomType::Silicon => "Si",
            AtomType::Phosphorus => "P",
            AtomType::Sulfur => "S",
            AtomType::Chlorine => "Cl",
            AtomType::Argon => "Ar",
            AtomType::Potassium => "K",
            AtomType::Calcium => "Ca",
            AtomType::Scandium => "Sc",
            AtomType::Titanium => "Ti",
            AtomType::Vanadium => "V",
            AtomType::Chromium => "Cr",
            AtomType::Manganese => "Mn",
            AtomType::Iron => "Fe",
            AtomType::Cobalt => "Co",
            AtomType::Nickel => "Ni",
            AtomType::Copper => "Cu",
            AtomType::Zinc => "Zn",
            AtomType::Gallium => "Ga",
            AtomType::Germanium => "Ge",
            AtomType::Arsenic => "As",
            AtomType::Selenium => "Se",
            AtomType::Bromine => "Br",
            AtomType::Krypton => "Kr",
            AtomType::Rubidium => "Rb",
            AtomType::Strontium => "Sr",
            AtomType::Yttrium => "Y",
            AtomType::Zirconium => "Zr",
            AtomType::Niobium => "Nb",
            AtomType::Molybdenum => "Mo",
            AtomType::Technetium => "Tc",
            AtomType::Ruthenium => "Ru",
            AtomType::Rhodium => "Rh",
            AtomType::Palladium => "Pd",
            AtomType::Silver => "Ag",
            AtomType::Cadmium => "Cd",
            AtomType::Indium => "In",
            AtomType::Tin => "Sn",
            AtomType::Antimony => "Sb",
            AtomType::Tellurium => "Te",
            AtomType::Iodine => "I",
            AtomType::Xenon => "Xe",
            AtomType::Cesium => "Cs",
            AtomType::Barium => "Ba",
            AtomType::Lanthanum => "La",
            AtomType::Cerium => "Ce",
            AtomType::Praseodymium => "Pr",
            AtomType::Neodymium => "Nd",
            AtomType::Promethium => "Pm",
            AtomType::Samarium => "Sm",
            AtomType::Europium => "Eu",
            AtomType::Gadolinium => "Gd",
            AtomType::Terbium => "Tb",
            AtomType::Dysprosium => "Dy",
            AtomType::Holmium => "Ho",
            AtomType::Erbium => "Er",
            AtomType::Thulium => "Tm",
            AtomType::Ytterbium => "Yb",
            AtomType::Lutetium => "Lu",
            AtomType::Hafnium => "Hf",
            AtomType::Tantalum => "Ta",
            AtomType::Tungsten => "W",
            AtomType::Rhenium => "Re",
            AtomType::Osmium => "Os",
            AtomType::Iridium => "Ir",
            AtomType::Platinum => "Pt",
            AtomType::Gold => "Au",
            AtomType::Mercury => "Hg",
            AtomType::Thallium => "Tl",
            AtomType::Lead => "Pb",
            AtomType::Bismuth => "Bi",
            AtomType::Polonium => "Po",
            AtomType::Astatine => "At",
            AtomType::Radon => "Rn",
            AtomType::Francium => "Fr",
            AtomType::Radium => "Ra",
            AtomType::Actinium => "Ac",
            AtomType::Thorium => "Th",
            AtomType::Protactinium => "Pa",
            AtomType::Uranium => "U",
            AtomType::Neptunium => "Np",
            AtomType::Plutonium => "Pu",
            AtomType::Americium => "Am",
            AtomType::Curium => "Cm",
            AtomType::Berkelium => "Bk",
            AtomType::Californium => "Cf",
            AtomType::Einsteinium => "Es",
            AtomType::Fermium => "Fm",
            AtomType::Mendelevium => "Md",
            AtomType::Nobelium => "No",
            AtomType::Lawrencium => "Lr",
            AtomType::Rutherfordium => "Rf",
            AtomType::Dubnium => "Db",
            AtomType::Seaborgium => "Sg",
            AtomType::Bohrium => "Bh",
            AtomType::Hassium => "Hs",
            AtomType::Meitnerium => "Mt",
            AtomType::Darmstadtium => "Ds",
            AtomType::Roentgenium => "Rg",
            AtomType::Copernicium => "Cn"
        }
    }
    
}

fn get_atomtype(elemental_symbol: &str) -> AtomType {
    match elemental_symbol {
        "H" => AtomType::Hydrogen,
        "He" => AtomType::Helium,
        "Li" => AtomType::Lithium,
        "Be" => AtomType::Beryllium,
        "B" => AtomType::Boron,
        "C" => AtomType::Carbon,
        "N" => AtomType::Nitrogen,
        "O" => AtomType::Oxygen,
        "F" => AtomType::Fluorine,
        "Ne" => AtomType::Neon,
        "Na" => AtomType::Sodium,
        "Mg" => AtomType::Magnesium,
        "Al" => AtomType::Aluminum,
        "Si" => AtomType::Silicon,
        "P" => AtomType::Phosphorus,
        "S" => AtomType::Sulfur,
        "Cl" => AtomType::Chlorine,
        "Ar" => AtomType::Argon,
        "K" => AtomType::Potassium,
        "Ca" => AtomType::Calcium,
        "Sc" => AtomType::Scandium,
        "Ti" => AtomType::Titanium,
        "V" => AtomType::Vanadium,
        "Cr" => AtomType::Chromium,
        "Mn" => AtomType::Manganese,
        "Fe" => AtomType::Iron,
        "Co" => AtomType::Cobalt,
        "Ni" => AtomType::Nickel,
        "Cu" => AtomType::Copper,
        "Zn" => AtomType::Zinc,
        "Ga" => AtomType::Gallium,
        "Ge" => AtomType::Germanium,
        "As" => AtomType::Arsenic,
        "Se" => AtomType::Selenium,
        "Br" => AtomType::Bromine,
        "Kr" => AtomType::Krypton,
        "Rb" => AtomType::Rubidium,
        "Sr" => AtomType::Strontium,
        "Y" => AtomType::Yttrium,
        "Zr" => AtomType::Zirconium,
        "Nb" => AtomType::Niobium,
        "Mo" => AtomType::Molybdenum,
        "Tc" => AtomType::Technetium,
        "Ru" => AtomType::Ruthenium,
        "Rh" => AtomType::Rhodium,
        "Pd" => AtomType::Palladium,
        "Ag" => AtomType::Silver,
        "Cd" => AtomType::Cadmium,
        "In" => AtomType::Indium,
        "Sn" => AtomType::Tin,
        "Sb" => AtomType::Antimony,
        "Te" => AtomType::Tellurium,
        "I" => AtomType::Iodine,
        "Xe" => AtomType::Xenon,
        "Cs" => AtomType::Cesium,
        "Ba" => AtomType::Barium,
        "La" => AtomType::Lanthanum,
        "Ce" => AtomType::Cerium,
        "Pr" => AtomType::Praseodymium,
        "Nd" => AtomType::Neodymium,
        "Pm" => AtomType::Promethium,
        "Sm" => AtomType::Samarium,
        "Eu" => AtomType::Europium,
        "Gd" => AtomType::Gadolinium,
        "Tb" => AtomType::Terbium,
        "Dy" => AtomType::Dysprosium,
        "Ho" => AtomType::Holmium,
        "Er" => AtomType::Erbium,
        "Tm" => AtomType::Thulium,
        "Yb" => AtomType::Ytterbium,
        "Lu" => AtomType::Lutetium,
        "Hf" => AtomType::Hafnium,
        "Ta" => AtomType::Tantalum,
        "W" => AtomType::Tungsten,
        "Re" => AtomType::Rhenium,
        "Os" => AtomType::Osmium,
        "Ir" => AtomType::Iridium,
        "Pt" => AtomType::Platinum,
        "Au" => AtomType::Gold,
        "Hg" => AtomType::Mercury,
        "Tl" => AtomType::Thallium,
        "Pb" => AtomType::Lead,
        "Bi" => AtomType::Bismuth,
        "Po" => AtomType::Polonium,
        "At" => AtomType::Astatine,
        "Rn" => AtomType::Radon,
        "Fr" => AtomType::Francium,
        "Ra" => AtomType::Radium,
        "Ac" => AtomType::Actinium,
        "Th" => AtomType::Thorium,
        "Pa" => AtomType::Protactinium,
        "U" => AtomType::Uranium,
        "Np" => AtomType::Neptunium,
        "Pu" => AtomType::Plutonium,
        "Am" => AtomType::Americium,
        "Cm" => AtomType::Curium,
        "Bk" => AtomType::Berkelium,
        "Cf" => AtomType::Californium,
        "Es" => AtomType::Einsteinium,
        "Fm" => AtomType::Fermium,
        "Md" => AtomType::Mendelevium,
        "No" => AtomType::Nobelium,
        "Lr" => AtomType::Lawrencium,
        "Rf" => AtomType::Rutherfordium,
        "Db" => AtomType::Dubnium,
        "Sg" => AtomType::Seaborgium,
        "Bh" => AtomType::Bohrium,
        "Hs" => AtomType::Hassium,
        "Mt" => AtomType::Meitnerium,
        "Ds" => AtomType::Darmstadtium,
        "Rg" => AtomType::Roentgenium,
        "Cn" => AtomType::Copernicium,
        _ => AtomType::Hydrogen
    }
}


pub struct Atom {
    atomtype: AtomType,
    mass: f64,
    electrons: i32,
}

impl Atom {
    pub fn new(atomtype: AtomType) -> Atom {
        let mass = atomtype.get_mass();
        let electrons = 0;
        Atom { atomtype, mass, electrons }
    }

    pub fn new_with_symbol(elemental_symbol: &str) -> Atom{
        let atomtype = get_atomtype(elemental_symbol);
        let mass = atomtype.get_mass();
        let electrons = 0;
        Atom { atomtype, mass, electrons }
    }

    
    pub fn get_elemental_symbol(&self) -> &str {
        AtomType::get_elemental_symbol(&self.atomtype)
    }
    
    pub fn get_mass(&self) -> f64 {
        self.mass
    }
    
    pub fn get_electrons(&self) -> i32 {
        self.electrons
    }
}

