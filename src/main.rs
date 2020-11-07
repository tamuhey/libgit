use git2::Repository;
fn main() {
    let repo = Repository::open(".").unwrap();
    let index = repo.index().unwrap();
    for ent in index.iter() {
        let p = String::from_utf8(ent.path).unwrap();
        println!("{} {} {}", p, ent.ctime.seconds(), ent.mtime.seconds());
    }
}
