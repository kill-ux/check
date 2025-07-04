
fn brain_fuck(str: &str) {

}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}