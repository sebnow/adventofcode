#[derive(Debug)]
struct Answer {
    steps: i32,
}

fn answer() -> Answer {
    Answer{steps: 0}
}

fn main() {
    println!("{:?}", answer());
}
