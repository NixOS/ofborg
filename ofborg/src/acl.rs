use crate::systems::System;

pub struct ACL {
    trusted_users: Vec<String>,
    known_users: Vec<String>,
    repos: Vec<String>,
}

impl ACL {
    pub fn new(
        repos: Vec<String>,
        mut trusted_users: Vec<String>,
        mut known_users: Vec<String>,
    ) -> ACL {
        trusted_users
            .iter_mut()
            .map(|x| *x = x.to_lowercase())
            .last();
        known_users.iter_mut().map(|x| *x = x.to_lowercase()).last();

        ACL {
            trusted_users,
            known_users,
            repos,
        }
    }

    pub fn is_repo_eligible(&self, name: &str) -> bool {
        self.repos.contains(&name.to_lowercase())
    }

    pub fn build_job_architectures_for_user_repo(&self, user: &str, repo: &str) -> Vec<System> {
        if self.can_build_unrestricted(user, repo) {
            vec![
                System::X8664Darwin,
                System::X8664Linux,
                System::Aarch64Linux,
            ]
        } else {
            // allow everybody to issue aarch64-linux and x8664-linux builds
            vec![System::X8664Linux, System::Aarch64Linux]
        }
    }

    pub fn build_job_destinations_for_user_repo(
        &self,
        user: &str,
        repo: &str,
    ) -> Vec<(Option<String>, Option<String>)> {
        self.build_job_architectures_for_user_repo(user, repo)
            .iter()
            .map(|system| system.as_build_destination())
            .collect()
    }

    pub fn can_build_restricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        self.known_users.contains(&user.to_lowercase())
    }

    pub fn can_build_unrestricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() == "nixos/nixpkgs" {
            self.trusted_users.contains(&user.to_lowercase())
        } else {
            user == "grahamc"
        }
    }
}
