pub enum OperatingSystem {
    Debian,
}

pub struct DependenciesActions {
    pub install: Vec<String>,
    pub uninstall: Vec<String>,
}

impl DependenciesActions {
    pub fn get(&self, action: &str) -> Option<&Vec<String>>{
        match action {
            "install" => Some(&self.install),
            "uninstall" => Some(&self.uninstall),
            _ => None,
        }
    }
}
