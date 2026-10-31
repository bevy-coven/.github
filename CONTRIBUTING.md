# Contributing to a Bevy Coven crate

This file applies to every repository in [bevy-coven](https://github.com/bevy-coven) that has no `CONTRIBUTING.md` of its own.

## Issues

- Label a defect `bug` and a request `enhancement`.
- Say what you expected, what happened, and the smallest way to see it: a `cargo run --example` line, or a failing test.
- A game that needs something from a crate asks for it with the labels `consumer` and `consumer:<game>`, and says what it needs and why, the current behaviour, the proposed API, the edge cases, and an acceptance test the crate can run on its own.

## Pull requests

- CI runs on self-hosted runners, so a maintainer approves the run of a first-time contributor's pull request before it starts. Expect a short wait.
- `cargo test` must pass. Add a test that fails without your change.
- Keep one change per pull request.

## Docs

A change a user can see updates the README and `docs/` in the same pull request, following [DOCUMENTATION.md](DOCUMENTATION.md). A feature is not done until its README line and, if it is visible, its GIF exist.

## Licence

Every crate is dual-licensed under MIT OR Apache-2.0. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
