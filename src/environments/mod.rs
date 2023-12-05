pub mod environments {
  use std::{io, fmt};

  pub struct InitialEnvrionments {
    root_directory: String,
    destination_direectory: String,
    should_delete_original_files: bool
  }

  impl InitialEnvrionments {
      pub fn new(root_directory: String, destination_directory: String, should_delete_original_files: bool) -> Self {
        InitialEnvrionments {
          root_directory: root_directory.trim().to_string(),
          destination_direectory: destination_directory.trim().to_string(),
          should_delete_original_files: should_delete_original_files
        }
      }

      pub fn get_root_directory(&self) -> &str {
        &self.root_directory
      }
      pub fn get_destination_directory(&self) -> &str {
        &self.destination_direectory
      }
      pub fn get_should_delete_original_files(&self) -> &bool {
        &self.should_delete_original_files
      }
  }

  impl fmt::Display for InitialEnvrionments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InitialEnvrionments {{\n\troot_directory: {}\n\tdestination_directory: {}\n\tshould_delete_original_files: {}\n}}",
            self.get_root_directory(), self.get_destination_directory(), self.get_should_delete_original_files()
        )
    }
  }


  pub fn get_initial_environments() -> InitialEnvrionments {
    println!("Root directory:");
    let mut root_directory: String = String::new();
    io::stdin().read_line(&mut root_directory).expect("error: unable to read root directory");
    println!("Root directory set as: {}", root_directory);

    println!("Destination directory:");
    let mut destination_directory: String = String::new();
    io::stdin().read_line(&mut destination_directory).expect("error: unable to read destination directory");
    println!("Destination directory: {}", destination_directory);

    println!("Should delete original files? (yes/NO)");
    let mut should_delete_original_files_input: String = String::new();
    io::stdin().read_line(&mut should_delete_original_files_input).expect("error: unable to read file deletion request");
    
    let mut should_delete_original_files: bool = false;

    if should_delete_original_files_input.trim() == "yes" {
      should_delete_original_files = true;
    }
    
    let initial_environments: InitialEnvrionments = InitialEnvrionments::new(root_directory, destination_directory, should_delete_original_files);

    initial_environments
  }
}