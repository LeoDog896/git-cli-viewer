use std::str::FromStr;
use std::fs;
use anyhow::{Result, Error, anyhow};
use git2::Repository;
use clap::Parser;
use tempfile::tempdir;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug, PartialEq, Eq, Clone)]
enum GitRepoType {
	GitHub,
	GitLab,
	BitBucket,
	Git
}

impl FromStr for GitRepoType {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Error> {
		let lowercase_string = s.to_lowercase();

		match &lowercase_string[..] {
			"github" => Ok(Self::GitHub),
			"gitlab" => Ok(Self::GitLab),
			"bitbucket" => Ok(Self::BitBucket),
			"git" => Ok(Self::Git),
			"" =>  Ok(Self::Git),
			_ => Err(anyhow!("Could not find git repo type"))
		}
	}
}

impl GitRepoType {
	fn to_git_url(&self, piece: &str) -> Result<String> {
		match self {
			Self::GitHub => Ok("https://github.com/".to_owned() + piece),
			Self::GitLab => Ok("https://gitlab.com/".to_owned() + piece),
			Self::Git => Ok(piece.to_owned()),
			Self::BitBucket => Ok("https://bitbucket.com".to_owned() + piece)
		}
	}
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    git_type: GitRepoType,
    /// The URL to see
    url: String,
}

fn main() -> Result<()> {

	let args = Cli::parse();

    let temp_dir = tempdir()?;

	let temp_dir_path = temp_dir.path();

	let url = args.git_type.to_git_url(&args.url)?;
	match Repository::clone(&url, temp_dir_path) {
		Ok(repo) => repo,
		Err(e) => panic!("failed to clone: {}", e),
	};

    let paths = fs::read_dir(temp_dir_path)?
		.filter_map(|dir| dir.ok())
		.map(|dir| dir.path())
		.map(|dir| dir.file_name().map(|it| it.to_os_string()))
		.flatten()
		.map(|dir| dir.to_str().map(|it| it.to_string()))
		.flatten()
		.map(|dir| dir.to_string())
		.collect::<Vec<String>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&paths)
        .interact()
        .unwrap();


	temp_dir.close()?;

	Ok(())
}