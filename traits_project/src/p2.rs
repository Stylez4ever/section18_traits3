use std::fmt::{Debug, Display, Formatter, Result};

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter,"AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(formatter, "AppleType::Granny Smith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} for💰 {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        //write!(formatter, "Apple ::: [ Kind: {}, Price: {} ]", self.kind, self.price)
        formatter
        .debug_struct("** Apple **")
        .field("Kind", &self.kind)
        .field("Cost", &self.price).finish()
    }
}

fn practice_2() {

    let lunch_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.04,
    };

    let dinner_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.15,
    };

    println!("{:?}", lunch_snack);
    
    println!("{:?}", dinner_snack);
}
