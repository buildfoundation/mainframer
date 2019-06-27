use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Ignore {
    pub common_ignore_file: Option<PathBuf>,
    pub local_ignore_file: Option<PathBuf>,
    pub remote_ignore_file: Option<PathBuf>,
}

impl Ignore {
    pub fn from_working_dir(working_dir: &Path) -> Ignore {
        let mut common_ignore_file = working_dir.to_path_buf();
        common_ignore_file.push(".mainframer/ignore");

        let mut local_ignore_file = working_dir.to_path_buf();
        local_ignore_file.push(".mainframer/localignore");

        let mut remote_ignore_file = working_dir.to_path_buf();
        remote_ignore_file.push(".mainframer/remoteignore");

        Ignore {
            common_ignore_file: if common_ignore_file.exists() {
                Some(common_ignore_file.to_path_buf())
            } else {
                None
            },
            local_ignore_file: if local_ignore_file.exists() {
                Some(local_ignore_file.to_path_buf())
            } else {
                None
            },
            remote_ignore_file: if remote_ignore_file.exists() {
                Some(remote_ignore_file.to_path_buf())
            } else {
                None
            },
        }
    }
}
