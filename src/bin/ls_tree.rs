/// get paths in index and sort by chronogical order
use git2::{Commit, DiffOptions, ObjectType, Repository, Signature, Time};
use git2::{DiffFormat, Error, Pathspec};

fn main() {
    run().unwrap();
}

fn get_paths(repo: &Repository) -> Vec<String> {
    let index = repo.index().unwrap();
    index
        .iter()
        .map(|ent| String::from_utf8(ent.path).unwrap())
        .collect()
}

fn run() -> Result<(), Error> {
    let path = ".";
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    revwalk.push_head()?;

    // Prepare our diff options and pathspec matcher
    let mut paths: Vec<_> = get_paths(&repo)
        .into_iter()
        .map(|path| (path, None))
        .collect();
    let mut sorted_paths = vec![];
    for id in revwalk {
        let id = id?;
        let commit = repo.find_commit(id)?;
        let tree = commit.tree()?;
        paths = paths
            .into_iter()
            .filter_map(|(path, id)| {
                if let Ok(entry) = tree.get_path(path.as_ref()) {
                    if let Some(id) = id {
                        if entry.id() == id {
                            Some((path, Some(id)))
                        } else {
                            sorted_paths.push(path);
                            None
                        }
                    } else {
                        Some((path, Some(entry.id())))
                    }
                } else {
                    sorted_paths.push(path);
                    None
                }
            })
            .collect();
        if paths.len() == 0 {
            break;
        }
    }
    sorted_paths.extend(paths.into_iter().map(|(p, _)| p));
    for p in sorted_paths {
        println!("{}", p);
    }
    Ok(())
}
