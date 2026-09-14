mod definition;
mod first;
mod second;

fn main() {
    assert_eq!(first::VALUE, 7);
    assert_eq!(second::VALUE, 9);
}
