use std::fs;
use std::process::Command;

fn clean_output_files() {
  fs::remove_dir_all("output").expect("Fail to execute the command 'rm -r output'");
}

#[test]
fn test_main_with_args() {
  if fs::metadata("./output").is_ok() {
    clean_output_files();
  }
  let output = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("-d")
    .arg("./tests/inputsForTest/")
    .output()
    .expect("Fail to execute the command of main program");

  let stdout = String::from_utf8(output.stdout).expect("Saída não válida UTF-8");
  let expected_output = "Total of 1 file with the repeated name\n\nTotal of 4 files with '.png' extension\nTotal of 1 files with '.mp4' extension\n\n\nTotal of 5 files organized\n";

  assert!(output.status.success());
  assert!(stdout.contains(expected_output));
  assert!(fs::metadata("./output/filesWithRepeatedName.txt").is_ok());
  assert!(fs::metadata("./output/2024/pictureFrom2024.png").is_ok());
  assert!(fs::metadata("./output/2024/pictureFrom2024-2.png").is_ok());
  assert!(fs::metadata("./output/2024/pictureFrom2024-3.png").is_ok());
  assert!(fs::metadata("./output/2024/movieFrom2024.mp4").is_ok());

  clean_output_files();
}
