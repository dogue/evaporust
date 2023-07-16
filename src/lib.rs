use std::{
    env::set_current_dir,
    fs::read_dir,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct ProjectFinder {
    base_dir: PathBuf,
    pub projects: Vec<PathBuf>,
    exclude: Vec<String>,
    all_projects: bool,
}

impl ProjectFinder {
    pub fn new(base_dir: PathBuf, exclude: Vec<String>, all_projects: bool) -> Self {
        let projects = Vec::new();

        Self {
            base_dir,
            projects,
            exclude,
            all_projects,
        }
    }

    pub fn clean(&self) {
        for project in self.projects.iter() {
            if set_current_dir(project).is_err() {
                log::warn!("Failed to change directory to {:?}", project);
                continue;
            }

            if let Err(e) = Command::new("cargo")
                .arg("clean")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
            {
                log::error!("Failed to run cargo clean in {:?}: {}", project, e);
                continue;
            }
        }
    }

    pub fn walk(&mut self) -> std::io::Result<()> {
        let mut dirs: Vec<PathBuf> = Vec::new();
        dirs.push(self.base_dir.clone());

        'walk: while !dirs.is_empty() {
            let dir = match dirs.pop() {
                Some(d) => d,
                None => continue,
            };

            for path in self.exclude.iter() {
                if dir.to_string_lossy().contains(path) {
                    continue 'walk;
                }
            }

            // hacky workaround because fs::try_exists() is experimental
            let cleaned = match std::fs::metadata(dir.join("target")) {
                Ok(_) => false,
                Err(_) => {
                    if self.all_projects {
                        false
                    } else {
                        true
                    }
                }
            };

            for entry in read_dir(dir.clone())? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    dirs.push(path);
                    continue;
                }

                if path.file_name().unwrap() == "Cargo.toml" {
                    if !cleaned {
                        self.projects.push(dir.clone());
                    }
                }
            }
        }

        Ok(())
    }
}
