// Codeforces 1A

fn input() -> Vec<u64> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    let out: Vec<u64> = parts.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    out
}

pub fn submit() {
    let sizes = input();
    let m = sizes[0];
    let n = sizes[1];
    let a = sizes[2];

    let x = ceiling(m, a);
    let y = ceiling(n, a);
    println!("{}", x * y);
}

fn ceiling(x: u64, a: u64) -> u64 {
    (x + a - 1) / a
}
