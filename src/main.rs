mod environments;
mod files;

use crate::environments::environments::get_initial_environments;
use environments::environments::InitialEnvrionments;
use crate::files::file::explore_directory;

use std::path::Path;

fn main() {
    let initial_environments: InitialEnvrionments = get_initial_environments();
    let path = Path::new(initial_environments.get_root_directory());

    explore_directory(path)
}
