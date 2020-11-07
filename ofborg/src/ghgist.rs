use hubcaps::gists::{Gist, GistOptions};

pub struct Client<'a> {
    github: &'a hubcaps::Github,
}

impl<'a> Client<'a> {
    pub fn new(github: &'a hubcaps::Github) -> Self {
        Client { github }
    }

    pub fn create_gist(&self, gist: &GistOptions) -> hubcaps::Result<Gist> {
        self.github.gists().create(gist)
    }
}
