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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colors {
    Color(char),
}

impl<'a> Colors {
    pub fn connections(input_color: Colors) -> (Colors, Vec<&'a Colors>) {
        let found_pair = CONNECTIONS
            .iter()
            .find(|cnnct| input_color == cnnct[0] || input_color == cnnct[1])
            .unwrap();

        let [front, back] = found_pair;
        let filtered: Vec<_> = COLORS
            .iter()
            .filter(|&color| *color != *front && *color != *back)
            .collect();

        return (input_color, filtered);
    }

    pub fn is_opposite_side(color: &Colors, opp: &Colors) -> bool {
        let mut is_opposite;

        let pair = CONNECTIONS
            .iter()
            .find(|cnct| *color == cnct[0] || *color == cnct[1])
            .unwrap();

        is_opposite = pair.contains(color) && pair.contains(opp);
        println!("is opp {is_opposite} {:?} {:?}", color, opp);
        println!(
            "from pair: {:?} {:?}",
            pair.contains(color),
            pair.contains(opp)
        );

        return is_opposite;
    }

    pub fn get_opposite(color: &Colors) -> Colors {
        let pair = CONNECTIONS
            .iter()
            .find(|cnct| *color == cnct[0] || *color == cnct[1])
            .unwrap();

        return pair[1];
    }
}
