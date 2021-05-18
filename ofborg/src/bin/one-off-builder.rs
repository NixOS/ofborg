/// Build attributes within a Nixpkgs PR the way that ofBorg does
use ofborg::nix::Nix;
use ofborg::notifyworker::SimpleNotifyWorker;
use ofborg::tasks::build::BuildWorker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ofborg::setup_log();
    let current_system = "x86_64-linux";

    let attrs = vec!["pulseview".to_owned()];

    let nix = Nix::new(
        current_system.to_owned(),
        "daemon".to_owned(),
        90 * 60,
        None,
    );
    let p: std::path::PathBuf = "/tmp/ofborg".into();
    let cloner = ofborg::checkout::cached_cloner(&p);
    let worker = BuildWorker::new(
        cloner,
        nix,
        current_system.to_owned(),
        "one-off".to_string(),
    );
    let repo = ofborg::message::Repo {
        owner: "nixos".to_owned(),
        name: "nixpkgs".to_owned(),
        full_name: "NixOS/nixpkgs".to_owned(),
        clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
    };

    let mut dummy_receiver = ofborg::notifyworker::DummyNotificationReceiver::new();

    let pr = ofborg::message::Pr {
        target_branch: Some("nixos-unstable".to_owned()),
        number: 123170,
        head_sha: "ec28b5d7a54fd1f8c2eba5e4ba238769d281755e".to_owned(),
    };

    let job = ofborg::message::buildjob::BuildJob::new(
        repo,
        pr,
        ofborg::commentparser::Subset::Nixpkgs,
        /* attrs */ attrs,
        /* logs */ None,
        /* statusreport: */ None,
        /*request_id: */ "one-off".to_string(),
    );

    worker.consumer(&job, &mut dummy_receiver);

    for message in dummy_receiver.actions {
        use ofborg::worker::Action;
        match message {
            Action::Ack | Action::NackRequeue | Action::NackDump => println!("{:?}", message),
            Action::Publish(msg) => match msg.content_type {
                Some(x) if x == "application/json" => {
                    let data: serde_json::Value = serde_json::from_slice(&msg.content).unwrap();
                    println!("Action::Publish: {:?}", data);
                }
                _ => {
                    println!("Action::Publish: {:?}", msg);
                }
            },
        }
    }

    Ok(())
}
