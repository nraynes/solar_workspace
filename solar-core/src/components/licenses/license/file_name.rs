use crate::components::licenses::{LICENSE_PREFIX, license::License};

impl License {
    pub fn file_name(&self) -> String {
        format!("{}{}", LICENSE_PREFIX, self)
    }
}
