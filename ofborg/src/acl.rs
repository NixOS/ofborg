
pub struct ACL {
    trusted_users: Vec<String>,
    known_users: Vec<String>,
}

impl ACL {
    pub fn new(trusted_users: Vec<String>, known_users: Vec<String>) -> ACL {
        return ACL {
            trusted_users: trusted_users,
            known_users: known_users,
        };
    }

    pub fn can_build_restricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        return self.known_users.contains(&user.to_lowercase());
    }

    pub fn can_build_unrestricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        return self.trusted_users.contains(&user.to_lowercase());
    }
}
