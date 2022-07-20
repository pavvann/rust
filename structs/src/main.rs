struct Twitch {
    viewers: i32,
    color: String,
}
fn main() {
    let mut stream = Twitch {
        viewers: 2,
        color: String::from("yellow"),
    };
    println!("{}\n{}", stream.viewers, stream.color);
    stream.viewers = 1;
    println!("{}", stream.viewers);
}
