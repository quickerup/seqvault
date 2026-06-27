pub struct Sequence {
    pub steps: Vec<String>,
}

impl Sequence {
    pub fn new(steps: Vec<String>) -> Self {
        Self { steps }
    }
}
