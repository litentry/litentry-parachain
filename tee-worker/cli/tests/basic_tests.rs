use clap::Parser;
use litentry_cli::Cli;

fn init() {
	let _ = env_logger::try_init();
}

#[test]
fn test_version() {
	init();

	let res = Cli::try_parse_from(vec!["placeholder_cli_path", "--version"]);
	let _err = clap::Error::new(clap::error::ErrorKind::DisplayVersion);
	assert!(matches!(res, Err(_err)));
}

#[test]
fn test_help() {
	init();

	let res = Cli::try_parse_from(vec!["placeholder_cli_path", "--help"]);
	let _err = clap::Error::new(clap::error::ErrorKind::DisplayHelp);
	assert!(matches!(res, Err(_err)));
}
