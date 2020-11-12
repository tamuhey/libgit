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
    println!("{:?}", sorted_paths);
    println!("{:?}", paths);
    // let revwalk = revwalk
    //     .filter_map(|id| {
    //         let id = filter_try!(id);
    //         let commit = filter_try!(repo.find_commit(id));
    //         let parents = commit.parents().len();
    //         match commit.parents().len() {
    //             0 => {
    //                 let tree = filter_try!(commit.tree());
    //                 let flags = git2::PathspecFlags::NO_MATCH_ERROR;
    //                 if ps.match_tree(&tree, flags).is_err() {
    //                     return None;
    //                 }
    //             }
    //             _ => {
    //                 let m = commit.parents().all(|parent| {
    //                     match_with_parent(&repo, &commit, &parent, &mut diffopts).unwrap_or(false)
    //                 });
    //                 if !m {
    //                     return None;
    //                 }
    //             }
    //         }
    //         Some(Ok(commit))
    //     })
    //     .skip(args.flag_skip.unwrap_or(0))
    //     .take(args.flag_max_count.unwrap_or(!0));

    // // print!
    // for commit in revwalk {
    //     let commit = commit?;
    //     print_commit(&commit);
    //     if !args.flag_patch || commit.parents().len() > 1 {
    //         continue;
    //     }
    //     let a = if commit.parents().len() == 1 {
    //         let parent = commit.parent(0)?;
    //         Some(parent.tree()?)
    //     } else {
    //         None
    //     };
    //     let b = commit.tree()?;
    // let diff = repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts2))?;
    //     diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
    //         match line.origin() {
    //             ' ' | '+' | '-' => print!("{}", line.origin()),
    //             _ => {}
    //         }
    //         print!("{}", str::from_utf8(line.content()).unwrap());
    //         true
    //     })?;
    // }

    Ok(())
}
