//Codeforces 4A

pub struct Watermelon {
    kilos: u32
}

impl Watermelon {
    fn new(kilos: u32) -> Self {
        assert!(kilos >= 1 && kilos <= 100, "Kilos must be from 1 to 100");
        Self {
            kilos
        }
    }

    fn is_even(&self) -> bool {
        let modulus = self.kilos % 2;

        if modulus == 0 && self.kilos > 2 {
            true
        } else {
            false
        }
    }

    pub fn submit() {
        let weight = Self::input();
        let watermelon = Self::new(weight);

        if watermelon.is_even() {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    fn input() -> u32 {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();
        input.trim().parse::<u32>().unwrap()
    }
}
