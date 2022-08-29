#[derive(Default)]
pub struct Executor {
    pub(crate) id: String,
    pub(crate) name: String,
}

impl Executor {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
