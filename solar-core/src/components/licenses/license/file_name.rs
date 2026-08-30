use crate::components::licenses::license::License;

impl License {
    pub fn file_name(&self) -> String {
        format!("LICENSE-{}", self)
    }
}
