use minifetch::{get_distro};

fn main() {
    let distro = get_distro();
    println!("OS: {}", distro);
}
