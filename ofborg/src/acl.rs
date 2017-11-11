
pub struct ACL {
    authorized_users: Vec<String>,
}

impl ACL {
    pub fn new(authorized_users: Vec<String>) -> ACL {
        return ACL {
            authorized_users: authorized_users,
        }
    }

    pub fn can_build(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        return self.authorized_users.contains(&user.to_lowercase());
    }
}
