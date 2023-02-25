

struct GroceryItem {
    stock: i32,
    price: f64,
}

pub fn structs() {
    let cereal: GroceryItem = GroceryItem {
        stock: 10,
        price: 2.99
    };
    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);
    return
}
