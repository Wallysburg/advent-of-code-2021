pub fn main() {
    let input: Vec<(&str, usize)> = include_str!("../input.txt")
    .lines()
    .map(|i| -> (&str, usize) {
        let mut input= i.split_whitespace();
        let direction = input.next().unwrap();
        let amount = input.next().unwrap().parse::<usize>().unwrap();
        return (direction, amount);
    })
    .collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for x in input {
        let direction = x.0;
        let amount = x.1;
        match direction {
            "forward" => {
                horizontal_position += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => print!("fuck")
        }
    }

    println!(
        "{}",
        horizontal_position * depth
    );
}
