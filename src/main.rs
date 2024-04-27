mod integers;
mod github;
fn main() {
    integers::run_i();

    println!("\n");

    integers::run_u();

    println!("\n");

    println!("github repo: {}",github::repo());
}
