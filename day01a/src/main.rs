pub fn main() {
    let mut input = include_str!("../input.txt")
    .lines()
    .map(|i| i.parse::<usize>().unwrap());

    let mut prev = input.next().unwrap();
    let solution = input.fold(0, |acc, x| {
        if x > prev {
            prev = x;
            return acc + 1;
        } else {
            prev = x;
            return acc;
        }
    });

    println!(
        "{}",
        solution
    );
}