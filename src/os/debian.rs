use crate::{ DependenciesActions, OperatingSystem };

pub struct DebianDependencies {
    chrome: DependenciesActions,
    code: DependenciesActions,
    curl: DependenciesActions,
    docker: DependenciesActions,
    fish: DependenciesActions,
    git: DependenciesActions,
    htop: DependenciesActions,
    npm: DependenciesActions,
    r: DependenciesActions,
    rstudio: DependenciesActions
}

impl DebianDependencies {
    pub fn get(&self, dependency: &str) -> Option<&DependenciesActions> {
        match dependency {
            "google-chrome" => Some(&self.chrome),
            "code" => Some(&self.code),
            "curl" => Some(&self.curl),
            "docker" => Some(&self.docker),
            "fish" => Some(&self.fish),
            "git" => Some(&self.git),
            "htop" => Some(&self.htop),
            "npm" => Some(&self.npm),
            "R" => Some(&self.r),
            "rstudio" => Some(&self.rstudio),
            _ => None
        }
    }
}

impl DebianDependencies {
    pub fn for_distribution(distribution: OperatingSystem) -> Option<DebianDependencies> {
        match distribution {
            OperatingSystem::Debian => Some(DebianDependencies {
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
                docker: DependenciesActions {
                    install: vec![
                        "sudo apt install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common".to_string(),
                        "curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg".to_string(),
                        r#"echo \
                            "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
                            $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
                        "#.to_string(),
                        "sudo apt update".to_string(),
                        "sudo apt install -y docker-ce docker-ce-cli containerd.io".to_string(),
                    ],
                    uninstall: vec![
                        "sudo systemctl stop docker".to_string(),
                        "sudo apt remove -y docker-ce docker-ce-cli containerd.io".to_string(),
                        "sudo rm -rf /var/lib/docker".to_string(),
                        "sudo rm -rf /etc/docker".to_string(),
                        "sudo deluser $USER docker".to_string(),
                        "sudo rm /etc/apt/sources.list.d/docker.list".to_string(),
                        "sudo rm /usr/share/keyrings/docker-archive-keyring.gpg".to_string(),
                        "sudo apt-get -y autoremove".to_string()
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
                    // https://cran.rstudio.com/bin/linux/debian/
                    install: vec!["sudo apt install -y r-base r-base-dev".to_string()],
                    uninstall: vec![
                        "sudo apt remove -y r-base r-base-dev".to_string(),
                        "sudo apt-get -y purge r-base r-base-dev".to_string(),
                        "sudo apt-get -y autoremove".to_string(),
                        "sudo rm -rf /usr/lib/R".to_string()
                    ],
                },
                rstudio: DependenciesActions {
                    install: vec![
                        // https://posit.co/download/rstudio-desktop/
                        "sudo apt update".to_string(),
                        "wget https://download1.rstudio.org/electron/jammy/amd64/rstudio-2023.12.1-402-amd64.deb --output-document=rstudio.deb".to_string(),
                        "sudo apt install -y ./rstudio.deb".to_string(),
                        "sudo rm rstudio.deb".to_string()
                    ],
                    uninstall: vec![
                        "sudo apt remove -y rstudio".to_string(),
                        "sudo apt-get -y purge rstudio".to_string(),
                        "sudo apt -y autoremove".to_string(),
                        "sudo rm -rf ~/.config/rstudio".to_string(),
                    ],
                },
            }),
        }
    }
}
