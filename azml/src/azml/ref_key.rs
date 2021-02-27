//

#[derive(Clone, Debug)]
pub struct RefKey {
    pub azer_text: RefKeyAzerText,
}

#[derive(Clone, Debug)]
pub struct RefKeyAzerText {
    pub prefix: String,
}
