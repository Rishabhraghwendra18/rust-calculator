mod lib;

fn main() {
    let args = lib::take_user_input();
    lib::calculate(&args);
}
