//

#[derive(Clone, Debug)]
pub struct RefKey {
    // These attributes will be included in the ref key instances.
    // Only final attributes can be included.
    //
    //TODO: consider to include in the ID instead? this is designed
    // to be part of entity's identity. A real-world example would be
    // human biometrics, e.g., fingerprint and iris.
    pub included_attributes: Vec<RefKeyIncludedAttribute>,
}

#[derive(Clone, Debug)]
pub struct RefKeyIncludedAttribute {
    pub name: String,
}
