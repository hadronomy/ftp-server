# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/Hadronomy/ftp-server/releases/tag/v0.1.0) - 2024-05-08

First release of the project :confetti_ball::tada::star2:

### Added
- allow empty password
- require password on login
- add log on connection close
- improve data transfer commands
- add env filter to tracing
- add graceful shutdown
- add per-connection working directory
- remove `--data-port` from cli
- add basic `MLSD` command implementation
- make `FEAT` return list of reatures
- add year to `LIST` command
- mark `FEAT` command as unimplemented
- add missing ftp commands
- add syst command
- add wip interactive mode warning
- add `AsyncWrite` and `AsyncRead` for `DataConnection`
- add ftp transfer commands
- *(ftp-server)* add some missing commands
- add `port` argument description
- add `WIP` note to cli description
- add `port` argument
- *(ftp-server)* change description
- *(ftp-server)* add basic cli
- *(ftp-server)* update layout distribution
- *(ftp-server)* configure logger for `tui_logger`
- *(ftp-server)* start ratatui implementation
- *(ftp-server)* make logging non-blocking

### Fixed
- `STOR` not returning proper status code
- try to use local ip address for `PASV`
- `QUIT` command not being executed
- improve instrumentation wording to make sense
- remove instrumentation from `SYST` command
- transfer incomplete in large files
- make `LIST` command wait for connection
- add missing space in response
- report bind port on `PASV` and not `0`
- trim `LIST` lines on trace
- place date in proper order on `LIST` command
- set directory flag to `-` when file
- require dummy password before login
- use `address` instead of `addr` in `info!`
- improve debug warning message
- init tracing before anything
- *(ftp-server)* change start message to info
- render on first run without waiting for event

### Other
- enable publish in `Cargo.toml`
- enable publish
- revert "ci: remove `release-plz` workflow"
- remove `release-plz` workflow
- disable publish in `Cargo.toml`
- add `release-plz` workflow
- add `cargo-dist` workflow
- add repository to `Cargo.toml`
- add `.pdf` files to `.gitignore`
- add `.cast` files to `.gitignore`
- remove line comment
- move connection cancel logic to `Connection`
- remove dummy file
- fix formatting issues
- remove unnused imports
- fix styling issues
- fix formatting issues
- use `InnerConnectionRef` instead of `Arc<Mutex<InnerConnection>>`
- remove unnused imports
- remove unnused imports
- separate commands into modules ([#2](https://github.com/Hadronomy/ftp-server/pull/2))
- move `Command` to it's own module
- remove unnused imports
- add missing docs
- remove unnused code
- remove unnused imports
- remove old unnused code
- start clean up into modules
- update `Cargo.lock`
- add `README`
- *(ftp-server)* add `.gitignore` and `rust-toolchain.toml`
- *(ftp-server)* improve inline docs
- *(ftp-server)* remove unnused imports
- *(ftp-server)* fix styling issues
- remove unnused imports
- rename 'crates' directory into 'experiments'
