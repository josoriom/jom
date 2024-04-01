pub enum OperatingSystem {
    Debian,
    // Add other OS variants as needed
}

pub struct DependenciesActions {
    install: Vec<String>,
    uninstall: Vec<String>,
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

pub struct AvailableDependencies {
    fish: DependenciesActions,
    git: DependenciesActions,
    htop: DependenciesActions,
    npm: DependenciesActions,
    r: DependenciesActions,
}

impl AvailableDependencies {
    pub fn get(&self, dependency: &str) -> Option<&DependenciesActions> {
        match dependency {
            "fish" => Some(&self.fish),
            "git" => Some(&self.git),
            "htop" => Some(&self.htop),
            "npm" => Some(&self.npm),
            "R" => Some(&self.r),
            _ => None
        }
    }
}

impl AvailableDependencies {
    pub fn for_distribution(distribution: OperatingSystem) -> Option<AvailableDependencies> {
        match distribution {
            OperatingSystem::Debian => Some(AvailableDependencies {
                fish: DependenciesActions {
                    install: vec!["sudo apt install -y fish".to_string()],
                    uninstall: vec!["sudo apt remove -y fish".to_string()],
                },
                git: DependenciesActions {
                    install: vec!["sudo apt install -y git".to_string()],
                    uninstall: vec!["sudo apt remove -y git".to_string()],
                },
                htop: DependenciesActions {
                    install: vec!["sudo apt install -y htop".to_string()],
                    uninstall: vec!["sudo apt remove -y htop".to_string()],
                },
                npm: DependenciesActions {
                    install: vec!["sudo apt install -y npm".to_string()],
                    uninstall: vec!["sudo apt remove -y npm".to_string()],
                },
                r: DependenciesActions {
                    install: vec!["sudo apt install -y r-base".to_string()],
                    uninstall: vec!["sudo apt remove -y r-base".to_string()],
                },
            }),
        }
    }
}
