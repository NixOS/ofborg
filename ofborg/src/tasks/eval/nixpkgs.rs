use crate::checkout::CachedProjectCo;
use crate::commentparser::Subset;
use crate::commitstatus::CommitStatus;
use crate::evalchecker::EvalChecker;
use crate::message::buildjob::BuildJob;
use crate::message::evaluationjob::EvaluationJob;
use crate::tasks::eval::{EvaluationComplete, EvaluationStrategy, StepResult};
use crate::tasks::evaluate::update_labels;

use std::path::Path;

use hubcaps::issues::IssueRef;
use regex::Regex;
use uuid::Uuid;

const TITLE_LABELS: [(&str, &str); 4] = [
    ("bsd", "6.topic: bsd"),
    ("darwin", "6.topic: darwin"),
    ("macos", "6.topic: darwin"),
    ("cross", "6.topic: cross-compilation"),
];

fn label_from_title(title: &str) -> Vec<String> {
    let labels: Vec<_> = TITLE_LABELS
        .iter()
        .filter(|(word, _label)| {
            let re = Regex::new(&format!("\\b{word}\\b")).unwrap();
            re.is_match(title)
        })
        .map(|(_word, label)| (*label).into())
        .collect();

    labels
}

pub struct NixpkgsStrategy<'a> {
    job: &'a EvaluationJob,
    issue_ref: &'a IssueRef,
    touched_packages: Option<Vec<String>>,
}

impl<'a> NixpkgsStrategy<'a> {
    pub fn new(job: &'a EvaluationJob, issue_ref: &'a IssueRef) -> NixpkgsStrategy<'a> {
        Self {
            job,
            issue_ref,
            touched_packages: None,
        }
    }

    fn tag_from_title(&self) {
        let title = match async_std::task::block_on(self.issue_ref.get()) {
            Ok(issue) => issue.title.to_lowercase(),
            Err(_) => return,
        };

        let labels = label_from_title(&title);

        if labels.is_empty() {
            return;
        }

        update_labels(self.issue_ref, &labels, &[]);
    }

    fn check_outpaths_before(&mut self, _dir: &Path) -> StepResult<()> {
        Ok(())
    }

    fn check_outpaths_after(&mut self) -> StepResult<()> {
        Ok(())
    }

    fn queue_builds(&self) -> StepResult<Vec<BuildJob>> {
        if let Some(ref possibly_touched_packages) = self.touched_packages {
            let mut try_build = possibly_touched_packages
                .iter()
                .flat_map(|pkg| vec![pkg.clone(), pkg.clone() + ".passthru.tests"].into_iter())
                .collect::<Vec<_>>();
            try_build.sort();
            try_build.dedup();

            if !try_build.is_empty() && try_build.len() <= 20 {
                // In the case of trying to merge master in to
                // a stable branch, we don't want to do this.
                // Therefore, only schedule builds if there
                // less than or exactly 20
                Ok(vec![BuildJob::new(
                    self.job.repo.clone(),
                    self.job.pr.clone(),
                    Subset::Nixpkgs,
                    try_build,
                    None,
                    None,
                    Uuid::new_v4().to_string(),
                )])
            } else {
                Ok(vec![])
            }
        } else {
            Ok(vec![])
        }
    }
}

impl<'a> EvaluationStrategy for NixpkgsStrategy<'a> {
    fn pre_clone(&mut self) -> StepResult<()> {
        self.tag_from_title();
        Ok(())
    }

    fn on_target_branch(&mut self, dir: &Path, status: &mut CommitStatus) -> StepResult<()> {
        status.set_with_description(
            "Checking original out paths",
            hubcaps::statuses::State::Pending,
        )?;
        self.check_outpaths_before(dir)?;

        Ok(())
    }

    fn after_fetch(&mut self, co: &CachedProjectCo) -> StepResult<()> {
        self.touched_packages = Some(parse_commit_messages(
            &co.commit_messages_from_head(&self.job.pr.head_sha)
                .unwrap_or_else(|_| vec!["".to_owned()]),
        ));

        Ok(())
    }

    fn after_merge(&mut self, status: &mut CommitStatus) -> StepResult<()> {
        status.set_with_description("Checking new out paths", hubcaps::statuses::State::Pending)?;
        self.check_outpaths_after()?;

        Ok(())
    }

    fn evaluation_checks(&self) -> Vec<EvalChecker> {
        vec![]
    }

    fn all_evaluations_passed(
        &mut self,
        status: &mut CommitStatus,
    ) -> StepResult<EvaluationComplete> {
        status.set_with_description(
            "Calculating Changed Outputs",
            hubcaps::statuses::State::Pending,
        )?;

        let builds = self.queue_builds()?;
        Ok(EvaluationComplete { builds })
    }
}

fn parse_commit_messages(messages: &[String]) -> Vec<String> {
    messages
        .iter()
        .filter_map(|line| {
            // Convert "foo: some notes" in to "foo"
            line.split_once(':').map(|(pre, _)| pre.trim())
        })
        // NOTE: This transforms `{foo,bar}` into `{{foo,bar}}` and `foo,bar` into `{foo,bar}`,
        // which allows both the old style (`foo,bar`) and the new style (`{foo,bar}`) to expand to
        // `foo` and `bar`.
        .flat_map(|line| brace_expand::brace_expand(&format!("{{{line}}}")))
        .map(|line| line.trim().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    trait PipeSort<T: Ord>: Sized + AsMut<[T]> {
        fn sorted(mut self) -> Self {
            self.as_mut().sort();
            self
        }
    }
    impl<T: Ord, L: Sized + AsMut<[T]>> PipeSort<T> for L {}

    #[test]
    fn test_parse_commit_messages() {
        let expect: Vec<&str> = vec![
            "firefox-esr",
            "firefox",
            "firefox",
            "buildkite-agent",
            "python.pkgs.ptyprocess",
            "python.pkgs.ptyprocess",
            "android-studio-preview",
            "foo",
            "bar",
            "firefox",
            "firefox-bin",
            "firefox-beta",
            "firefox-beta-bin",
            "librewolf",
        ];
        assert_eq!(
            parse_commit_messages(
                &"
              firefox{-esr,}: fix failing build due to the google-api-key
              Merge pull request #34483 from andir/dovecot-cve-2017-15132
              firefox: enable official branding
              Merge pull request #34442 from rnhmjoj/virtual
              buildkite-agent: enable building on darwin
              python.pkgs.ptyprocess: 0.5 -> 0.5.2
              python.pkgs.ptyprocess: move expression
              Merge pull request #34465 from steveeJ/steveej-attempt-qtile-bump-0.10.7
              android-studio-preview: 3.1.0.8 -> 3.1.0.9
              Merge pull request #34188 from dotlambda/home-assistant
              Merge pull request #34414 from dotlambda/postfix
              foo,bar: something here: yeah
              firefox{,-beta}{,-bin}, librewolf: blah blah blah
            "
                .lines()
                .map(|l| l.to_owned())
                .collect::<Vec<String>>(),
            ),
            expect
        );
    }

    #[test]
    fn test_label_platform_from_title() {
        assert_eq!(
            label_from_title("libsdl: 1.0.0 -> 1.1.0"),
            Vec::<String>::new()
        );
        assert_eq!(
            label_from_title("darwini: init at 1.0.0"),
            Vec::<String>::new()
        );
        assert_eq!(
            label_from_title("sigmacosine: init at 1.0.0"),
            Vec::<String>::new()
        );
        assert_eq!(
            label_from_title("fix build on bsd"),
            vec![String::from("6.topic: bsd")]
        );
        assert_eq!(
            label_from_title("fix build on darwin"),
            vec![String::from("6.topic: darwin")]
        );
        assert_eq!(
            label_from_title("fix build on macos"),
            vec![String::from("6.topic: darwin")]
        );
        assert_eq!(
            label_from_title("fix build on bsd and darwin").sorted(),
            vec![
                String::from("6.topic: darwin"),
                String::from("6.topic: bsd")
            ]
            .sorted()
        );
        assert_eq!(
            label_from_title("pkg: fix cross"),
            vec![String::from("6.topic: cross-compilation")]
        );
        assert_eq!(
            label_from_title("pkg: fix cross-compilation"),
            vec![String::from("6.topic: cross-compilation")]
        );
    }
}
