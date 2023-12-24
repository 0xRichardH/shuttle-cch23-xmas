use std::{fs, io::Cursor, path::PathBuf};

use anyhow::Context;
use axum::body;
use git2::Repository;
use tar::Archive;
use tempfile::TempDir;

use crate::prelude::*;

pub async fn count_archive_files(bytes: body::Bytes) -> Result<String> {
    let mut a = Archive::new(Cursor::new(bytes));
    let counter = a
        .entries()
        .map_err(|e| anyhow::anyhow!("failed to count files: {e}"))?
        .count();
    Ok(counter.to_string())
}

pub async fn count_archive_files_size(bytes: body::Bytes) -> Result<String> {
    let mut a = Archive::new(Cursor::new(bytes));
    let entries = a
        .entries()
        .map_err(|e| anyhow::anyhow!("failed to count files: {e}"))?;
    let mut size = 0;
    for file in entries {
        size += file?.size();
    }

    Ok(size.to_string())
}

pub async fn get_cookie_from_archive_file(bytes: body::Bytes) -> Result<String> {
    // save the archive to a temporary directory
    let tmp = TempDir::new()?;
    let path = tmp.path().join(f!("repo/{}", uuid::Uuid::new_v4()));
    let mut a = Archive::new(Cursor::new(bytes));
    a.unpack(&path).context("failed to unpack archive")?;

    // open the repository and switch to the christmas branch
    let repo = Repository::open(&path).context("open repository")?;
    checkout_to_branch_christmas(&repo).context("checkout to christmas")?;

    // traverse the commit history
    let revwalk = prepare_revwalk_for_traversal(&repo).context("prepare revwalk")?;
    for oid in revwalk {
        let oid = oid.context("get commit")?;
        let commit = repo.find_commit(oid).context("find git commmit")?;
        tracing::trace!("commit: {} {}", commit.author(), commit.id());

        // checkout the commit
        checkout_to_commit(&repo, &commit)?;

        // check if the cookie exists
        let has_found_cookie = has_found_cookie(&path).context("check for cookie")?;
        if has_found_cookie {
            let author = commit.author();
            let name = author.name().context("get author name")?;
            return Ok(f!("{} {}", name, commit.id()));
        }
    }

    Ok(String::from("No cookie found"))
}

fn checkout_to_branch_christmas(repo: &git2::Repository) -> Result<()> {
    let branch = repo
        .find_branch("christmas", git2::BranchType::Local)
        .context("find christmas branch")?;
    let Some(branch_name) = branch.name().context("get branch name")? else {
        return Err(AppError::Internal(anyhow::anyhow!(
            "get branch name failed"
        )));
    };
    repo.set_head(f!("refs/heads/{branch_name}").as_str())
        .context("set head to christmas")?;

    Ok(())
}

fn prepare_revwalk_for_traversal(repo: &git2::Repository) -> Result<git2::Revwalk> {
    let mut revwalk = repo.revwalk().context("get commit history")?;
    revwalk
        .set_sorting(git2::Sort::TIME)
        .context("set reop revwalk sorting")?;
    revwalk.push_head().context("push head to revwalk")?;

    Ok(revwalk)
}

fn checkout_to_commit(repo: &git2::Repository, commit: &git2::Commit) -> Result<()> {
    let tree = commit.tree().context("get commit tree")?;
    repo.checkout_tree(
        tree.as_object(),
        Some(&mut git2::build::CheckoutBuilder::new().force()),
    )
    .context("checkout tree")?;
    repo.set_head_detached(commit.id())
        .context("set head detached")?;

    Ok(())
}

fn has_found_cookie(path: &PathBuf) -> Result<bool> {
    if !path.is_dir() {
        return Ok(false);
    }

    for entry in fs::read_dir(path).context("read dir")? {
        let path = entry?.path();
        if path.file_name() == Some(".git".as_ref()) {
            continue;
        }

        if path.is_dir() {
            let result = has_found_cookie(&path)?;
            if result {
                return Ok(true);
            } else {
                continue;
            }
        }

        if path.file_name() == Some("santa.txt".as_ref()) {
            let content = fs::read_to_string(path).context("read santa.txt")?;
            return Ok(content.contains("COOKIE"));
        }
    }

    Ok(false)
}
