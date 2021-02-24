//

#[derive(Clone, Debug)]
pub struct RefKey {
    pub azrs: RefKeyAzis,
}

#[derive(Clone, Debug)]
pub struct RefKeyAzis {
    pub prefix: String,
}
