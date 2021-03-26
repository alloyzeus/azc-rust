//

#[derive(Clone, Debug)]
pub struct RefKey {
    pub azid_text: RefKeyAzidText,
}

#[derive(Clone, Debug)]
pub struct RefKeyAzidText {
    pub prefix: String,
}
