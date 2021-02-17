use hubcaps::gists::{Content, Gist, GistOptions};
use std::collections::HashMap;

pub trait Client {
    fn create_gist(&self, gist: &GistOptions) -> hubcaps::Result<Gist>;

    fn create_gist_with_content(&self, name: &str, contents: String) -> hubcaps::Result<Gist> {
        let mut files: HashMap<String, Content> = HashMap::new();
        files.insert(
            name.to_string(),
            hubcaps::gists::Content {
                filename: Some(name.to_string()),
                content: contents,
            },
        );

        self.create_gist(&GistOptions {
            description: None,
            public: Some(true),
            files,
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use hubcaps::gists::GistFile;
    use hubcaps::users::User;
    use std::cell::RefCell;
    use std::collections::HashSet;

    struct GistMock(RefCell<Vec<hubcaps::Result<Gist>>>);

    impl Client for GistMock {
        fn create_gist(&self, opts: &GistOptions) -> hubcaps::Result<Gist> {
            let gist = self
                .0
                .borrow_mut()
                .pop()
                .expect("GistMock.create_gist called too many times")?;
            assert_eq!(opts.files.len(), gist.files.len());
            assert_eq!(
                opts.files.iter().map(|(k, _)| k).collect::<HashSet<_>>(),
                gist.files.iter().map(|(k, _)| k).collect::<HashSet<_>>()
            );
            Ok(gist)
        }
    }

    #[test]
    fn test_create_gist_with_content() {
        let mut expected_files = HashMap::new();
        expected_files.insert(
            String::from("example.txt"),
            GistFile {
                size: 0,
                raw_url: String::new(),
                content: Some(String::from("Hello World!")),
                content_type: String::new(),
                truncated: None,
                language: None,
            },
        );
        let client = GistMock(RefCell::new(vec![Ok(Gist {
            url: String::new(),
            forks_url: String::new(),
            commits_url: String::new(),
            id: String::new(),
            description: None,
            public: true,
            owner: User {
                login: String::from("johndoe"),
                id: 42,
                avatar_url: String::new(),
                gravatar_id: String::new(),
                url: String::new(),
                html_url: String::new(),
                followers_url: String::new(),
                following_url: String::new(),
                gists_url: String::new(),
                starred_url: String::new(),
                subscriptions_url: String::new(),
                organizations_url: String::new(),
                repos_url: String::new(),
                events_url: String::new(),
                received_events_url: String::new(),
                site_admin: false,
            },
            user: None,
            files: expected_files,
            truncated: false,
            comments: 0,
            comments_url: String::new(),
            html_url: String::from("https://example.org"),
            git_pull_url: String::new(),
            git_push_url: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
        })]));

        let result = client.create_gist_with_content("example.txt", String::from("Hello World!"));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().html_url,
            String::from("https://example.org")
        );
    }
}
