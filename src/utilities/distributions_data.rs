use std::collections::HashMap;

pub enum OperatingSystem {
    Debian,
    // Add other OS variants as needed
}

struct DependenciesActions {
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub enum Dependencies {}

pub struct AvailableDependencies {
    fish: DependenciesActions,
    git: DependenciesActions,
    htop: DependenciesActions,
    npm: DependenciesActions,
    python3: DependenciesActions,
    pip3: DependenciesActions,
    R: DependenciesActions,
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
                python3: DependenciesActions {
                    install: vec!["sudo apt install -y python3".to_string()],
                    uninstall: vec!["sudo apt remove -y python3.11 python3.11-minimal".to_string()],
                },
                pip3: DependenciesActions {
                    install: vec!["sudo apt install -y pip3".to_string()],
                    uninstall: vec!["sudo apt remove -y pip3".to_string()],
                },
                R: DependenciesActions {
                    install: vec!["sudo apt install -y r-base".to_string()],
                    uninstall: vec!["sudo apt remove -y r-base".to_string()],
                },
            }),
        }
    }
}

pub fn list_available_dist() {
    if let Some(datum) = AvailableDependencies::for_distribution(OperatingSystem::Debian) {
        if let Some(otherDatum) = datum["fish"] {
            println!("{}", otherDatum[])
        }
    };
}
