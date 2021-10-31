
#[derive(Clone, Debug)]
pub struct ScoreCategory {
    pub name: String,
    pub value: i32,
}

impl ScoreCategory {
    pub fn new(name: String, value: i32) -> ScoreCategory {
        ScoreCategory { name: name, value: value }
    }
}
