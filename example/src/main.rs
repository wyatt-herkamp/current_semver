use current_semver::{current_major, current_semver};
const CURRENT_MAJOR: usize = current_major!();
const CURRENT_MAJOR_U64: u64 = current_major!(as u64);
fn main() {
    let current_major = current_major!();
    println!("Current major: {}", current_major);
    let current_major_u64 = current_major!(as u64);
    println!("Current major: {}", current_major_u64);

    let current_semver = current_semver!();
    println!("Current semver: {}", current_semver);
}
