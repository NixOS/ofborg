use hubcaps::gists::{Gist, GistOptions};

pub trait Client {
    fn create_gist(&self, gist: &GistOptions) -> hubcaps::Result<Gist>;
}

pub struct Hubcaps<'a> {
    github: &'a hubcaps::Github,
}

impl<'a> Hubcaps<'a> {
    pub fn new(github: &'a hubcaps::Github) -> Self {
        Hubcaps { github }
    }
}

impl Client for Hubcaps<'_> {
    fn create_gist(&self, gist: &GistOptions) -> hubcaps::Result<Gist> {
        self.github.gists().create(gist)
    }
}
