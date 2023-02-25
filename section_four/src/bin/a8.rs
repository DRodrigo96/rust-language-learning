


enum Flavor {
    _Cherry,
    _Coke,
    Orange,
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn drink_charac(fl: Flavor, ou: f64) {
    let drink: Drink = Drink {flavor: fl, ounce: ou};
    
    match drink.flavor {
        Flavor::_Cherry => println!("drink flavor: cherry"),
        Flavor::_Coke   => println!("drink flavor: coke"),
        Flavor::Orange  => println!("drink flavor: orange"),
    }
    println!("drink ounces: {:?}", drink.ounce);
    return
}

fn main() {
    drink_charac(Flavor::Orange, 21.1);
}
