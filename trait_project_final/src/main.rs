use std::fmt::{Debug, Display, Formatter, Result};
use std::clone::Clone;


// Define a Drinkable trait with 3 required methods
trait Drinkable {
    fn consume(&mut self);

    fn get_data(&self) -> String;

    fn stats(&self) -> String {
        self.get_data()
    }

    //corrections
    //fn stats(&self) {
    //    println!("{}",self.get_data())
    //}    
}


// Define a Milk enum with 3 variants
#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

impl Display for Milk {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Milk::Whole => write!(formatter, "Whole milk 🥛"),
            Milk::Oat => write!(formatter, "Oat milk 🍶🌿"),
            Milk::Almond => write!(formatter, "Almond milk 🥜🌰")
        }
    }
}


// Define a Coffee struct with one generic `T`

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

// Define a `new` constructor function for Coffee that
//returns a new Coffee instance. The function should be
//available for any generic type T.
impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Coffee<T> {
        Coffee { kind, milk, ounces }
    }
}

// Define a manual Debug trait implementation for a Coffee struct.
//impl<T: fmt::Display> Debug for Coffee<T> {
impl<T: Debug> Debug for Coffee<T> {
    //fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    //    write!(formatter, "Coffee ::: [ Kind: {}, Milk: {}, Ounces: {}]",
    //            self.kind, self.milk, self.ounces
    // )
    //}
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.
        debug_struct("** Coffee **").
        field("Kind", &self.kind).
        field("Milk", &self.milk).
        field("Ounces", &self.ounces).finish()
    }

}    

impl<T: Display> Drinkable for Coffee<T> {

    fn consume(&mut self) {
        self.ounces = 0/self.ounces
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }

    
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64, 
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32,price: f64,flavor: &str) -> Soda {
        Soda {
            calories,
            price,
            flavor: flavor.to_string(),
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = self.percentage*0
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "**🍹 {} 🍹Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories.clone(),
            price: self.price.clone(),
            flavor: self.flavor.clone(),
            percentage: self.percentage.clone(),


            // Correction
            //calories: self.calories,
            //price: self.price,
            //flavor: self.flavor.clone(),
            //percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}




fn main() {
    let mut latte = Coffee::new("Latte", Milk::Whole, 3);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new(
        String::from("Cappuccino"), 
        Milk::Oat, 
        5);
    cappuccino.get_data();
    println!("{:?}", cappuccino.stats());

    println!();
    println!("**************************************************");
    println!();

    let pepsi = Soda::new(150, 20.00, "Cherry");
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("{}", pepsi.eq(&coke));

    coke.consume();
    println!("{:?}", coke);
}
