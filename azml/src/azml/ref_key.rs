//

#[derive(Clone, Debug)]
pub struct RefKey {
    pub azis: RefKeyAzis,
}

#[derive(Clone, Debug)]
pub struct RefKeyAzis {
    pub prefix: String,
}
