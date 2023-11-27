const CONNECTIONS: [[Colors; 2]; 3] = [
    [Colors::Color('W'), Colors::Color('Y')],
    [Colors::Color('B'), Colors::Color('G')],
    [Colors::Color('R'), Colors::Color('O')],
];
pub const COLORS: [Colors; 6] = [
    Colors::Color('W'),
    Colors::Color('Y'),
    Colors::Color('B'),
    Colors::Color('G'),
    Colors::Color('R'),
    Colors::Color('O'),
];

#[derive(Debug, PartialEq, Eq)]
pub enum Colors {
    Color(char),
}

impl<'a> Colors {
    pub fn connections(inputColor: Colors) -> (Colors, Vec<&'a Colors>) {
        let found_pair = CONNECTIONS
            .iter()
            .find(|cnnct| inputColor == cnnct[0] || inputColor == cnnct[1])
            .unwrap();

        let [front, back] = found_pair;
        let filtered: Vec<_> = COLORS
            .iter()
            .filter(|&color| *color != *front && *color != *back)
            .collect();

        return (inputColor, filtered);
    }
}
