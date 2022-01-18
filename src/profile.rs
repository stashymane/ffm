use std::path::Path;

pub struct Profile {
    path: String,
}

impl Profile {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn install(&self, package_path: &str) {}
    pub fn uninstall(&self, package_id: &str) {}
    pub fn enable(&self, package_id: &str) {}
    pub fn disable(&self, package_id: &str) {}
}
