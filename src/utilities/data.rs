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
    chrome: DependenciesActions,
    code: DependenciesActions,
    curl: DependenciesActions,
    fish: DependenciesActions,
    git: DependenciesActions,
    htop: DependenciesActions,
    npm: DependenciesActions,
    r: DependenciesActions,
}

impl AvailableDependencies {
    pub fn get(&self, dependency: &str) -> Option<&DependenciesActions> {
        match dependency {
            "google-chrome" => Some(&self.chrome),
            "code" => Some(&self.code),
            "curl" => Some(&self.curl),
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
                chrome: DependenciesActions {
                    install: vec![
                        r#"wget "https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb" --output-document=chrome.deb"#.to_string(),
                        "sudo apt install -y ./chrome.deb".to_string(),
                        "rm chrome.deb".to_string()
                    ],
                    uninstall: vec![
                        "sudo apt-get remove -y google-chrome-stable".to_string(),
                        "sudo apt-get purge -y google-chrome-stable".to_string(),
                        "sudo apt-get autoremove -y".to_string(),
                        "rm -y /etc/apt/sources.list.d/google-chrome.list".to_string()
                    ],
                },
                code: DependenciesActions {
                    install: vec![
                        r#"wget "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64" --output-document=vscode.deb"#.to_string(),
                        "sudo apt install ./vscode.deb".to_string(),
                        "rm vscode.deb".to_string()
                    ],
                    uninstall: vec![
                        "sudo apt-get remove -y code".to_string(),
                        "sudo apt-get purge -y code".to_string(),
                        "sudo apt-get autoremove".to_string()
                    ],
                },
                curl: DependenciesActions {
                    install: vec![
                        "sudo apt update".to_string(),
                        "sudo apt install curl".to_string(),
                    ],
                    uninstall: vec![
                        "sudo apt remove -y curl".to_string(),
                        "sudo apt purge -y curl".to_string(),
                        "sudo apt-get autoremove".to_string()
                    ],
                },
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
                    uninstall: vec![
                        "sudo apt remove -y r-base".to_string(),
                        "sudo apt-get -y purge r-base".to_string(),
                        "sudo apt-get -y autoremove".to_string(),
                        "sudo rm -rf /usr/lib/R".to_string()
                    ],
                },
            }),
        }
    }
}
