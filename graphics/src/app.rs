use crate::{start_editor, version::Version};

pub struct App {
    pub name: String,
    pub version: Version,
}

impl App {
    pub fn new(name: impl Into<String>, version: impl Into<Version>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn run(self) -> Self {
        println!("Running {} v{}", self.name, self.version);
        start_editor(self.name.as_str());
        self
    }
}
