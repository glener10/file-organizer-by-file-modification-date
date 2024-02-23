use std::ffi::OsStr;

use crate::organize::ExtensionCounter;

pub fn print_executing_log(count_files: i32, paths_len: usize, file_name: &OsStr) {
  let percent = ((count_files as f64 / paths_len as f64) * 100.0).round();
  println!(
    "{}% Concluded, File {} of {}  -  File Name: {:?}",
    percent, count_files, paths_len, file_name
  );
}

pub fn print_finish_total_log(count_files: i32, extension_counters: &Vec<ExtensionCounter>) {
  if count_files > 0 {
    for counter in extension_counters {
      println!(
        "Total of {} files with '.{}' extension",
        counter.count, counter.extension
      );
    }
    println!("\n\nTotal of {} files organized", count_files);
  }
}
