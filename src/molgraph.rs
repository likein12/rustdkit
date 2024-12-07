
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    C, N, O, F, Cl, Br,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
    // Aromaticなども本来必要
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub element: Element,
    pub atom_id: usize,
    // 座標なし（1D表現）とする
}

#[derive(Debug, Clone)]
pub struct Bond {
    pub atom1: usize,
    pub atom2: usize,
    pub order: BondOrder,
}

#[derive(Debug, Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub name: Option<String>,
}

impl Molecule {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
            name: None,
        }
    }

    pub fn add_atom(&mut self, element: Element) -> usize {
        let atom_id = self.atoms.len();
        self.atoms.push(Atom { element, atom_id });
        atom_id
    }

    pub fn add_bond(&mut self, a1: usize, a2: usize, order: BondOrder) {
        self.bonds.push(Bond { atom1: a1, atom2: a2, order });
    }
}

/// 非常に簡易なSMILESパーサ
/// 入力例: "CCO" -> C - C - O
///        "CCl" -> C - Cl
pub fn parse_smiles(smiles: &str) -> Molecule {
    let mut mol = Molecule::new();

    // SMILESをトークンに分解する：
    // 簡易的には、1文字または2文字で元素を判別する。
    // Cl, Brなど2文字のハロゲン元素に対応。
    let chars: Vec<char> = smiles.chars().collect();
    let mut i = 0;
    let mut prev_atom: Option<usize> = None;

    while i < chars.len() {
        let c = chars[i];
        let (element, consumed) = match c {
            'C' => (Element::C, 1),
            'N' => (Element::N, 1),
            'O' => (Element::O, 1),
            'F' => (Element::F, 1),
            'B' => {
                // 次の文字が'r'ならBrとみなす
                if i+1 < chars.len() && chars[i+1] == 'r' {
                    (Element::Br, 2)
                } else {
                    (Element::Unknown, 1) // 本来はB単独はBoronだがここでは省略
                }
            },
            'C' if i+1 < chars.len() && chars[i+1] == 'l' => (Element::Cl, 2),
            _ => {
                // 次が'l'かどうかをチェック
                if c == 'C' && i+1 < chars.len() && chars[i+1] == 'l' {
                    (Element::Cl, 2)
                } else {
                    // 未知の元素(本来はエラー)
                    (Element::Unknown, 1)
                }
            }
        };

        // 原子追加
        let current_atom = mol.add_atom(element);

        // 前の原子と結合
        if let Some(pa) = prev_atom {
            // 結合記号を全て省略または単結合固定
            mol.add_bond(pa, current_atom, BondOrder::Single);
        }

        prev_atom = Some(current_atom);

        i += consumed;
    }

    mol
}

/// SMILES出力ルーチン(非常に単純な直鎖用)
/// この例では、直列につながった分子のみ正しく出力できることを仮定している。
/// 複雑な分枝や環がある場合は不正確。
pub fn to_smiles(mol: &Molecule) -> String {
    // atomsが順番に結合していると仮定し、element名を連結する。
    // Cl, Br以外は1文字、Cl,Brは2文字。ここではatom_id順に並べ、bondsを辿る単純実装。
    // 実際にはbond情報を用いて線形順序を決定する必要があるが、
    // ここでは入力が線型構造のみであると仮定。
    let mut s = String::new();
    for atom in &mol.atoms {
        s.push_str(&match atom.element {
            Element::C => "C".to_string(),
            Element::N => "N".to_string(),
            Element::O => "O".to_string(),
            Element::F => "F".to_string(),
            Element::Cl => "Cl".to_string(),
            Element::Br => "Br".to_string(),
            Element::Unknown => "*".to_string(),
        });
    }
    s
}

