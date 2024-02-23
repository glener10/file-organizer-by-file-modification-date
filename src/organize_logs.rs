use std::ffi::OsStr;

pub fn print_executing_log(count_files: i32, paths_len: usize, file_name: &OsStr) {
  let percent = ((count_files as f64 / paths_len as f64) * 100.0).round();
  println!(
    "{}% Concluded, File {} of {}  -  File Name: {:?}",
    percent, count_files, paths_len, file_name
  );
}
