use std::io::Read;

mod ubig_int;
mod big_int;
mod decimal_number;

fn read_number_file(file_name: &str) -> decimal_number::DecimalNumber {
  let mut s = String::new();
  std::fs::File::open(file_name)
    .expect("File not found")
    .read_to_string(&mut s)
    .expect("Failed to read file");
  decimal_number::DecimalNumber::new(s.trim())
}

fn main() {
  let file_number = std::time::Instant::now();
  let pi = read_number_file("./PI.number");
  let tau = read_number_file("./TAU.number");
  println!("Elapsed time reading numbers: {:?}", file_number.elapsed());
  let pi_pi = std::time::Instant::now();
  let pi_2 = pi.add(&pi);
  println!("Elapsed time adding Pi + Pi: {:?}", pi_pi.elapsed());
  let tau_pi = std::time::Instant::now();
  let tau_pi2 = tau.equals(&pi_2);
  println!("Tau equals 2 * Pi: {}", tau_pi2);
  println!("Elapsed time validate: {:?}", tau_pi.elapsed());
}
