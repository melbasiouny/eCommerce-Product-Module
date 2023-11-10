
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 73,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.73.0 (cc66ad468 2023-10-03)".to_owned(),
                    commit_hash: Some("cc66ad468955717ab92600c770da8c1601a4ff33".to_owned()),
                    commit_date: Some("2023-10-03".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            