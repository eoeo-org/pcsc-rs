use git2::{DescribeFormatOptions, DescribeOptions, Error, Repository};
use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    match get_git_describe_result() {
        Ok(result) => println!("cargo::rustc-env=GIT_DESCRIBE={result}"),
        Err(e) => println!("cargo::warning={}", e),
    }

    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
    }
    Ok(())
}

fn get_git_describe_result() -> Result<String, Error> {
    let repo = Repository::open(".")?;
    let describe = repo.describe(
        DescribeOptions::new()
            .describe_tags()
            .show_commit_oid_as_fallback(true),
    )?;
    let formatted = describe.format(Some(
        DescribeFormatOptions::new()
            .always_use_long_format(true)
            .dirty_suffix("-dirty"),
    ))?;
    Ok(formatted)
}
