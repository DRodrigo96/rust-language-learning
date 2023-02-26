

struct GroceryItem {
    quantity: i32,
    id: i32
}

fn show_quantity(item: &GroceryItem) -> () {
    println!("item-quantity: {:?}", item.quantity)
}

fn show_id(item: &GroceryItem) -> () {
    println!("item-id: {:?}", item.id)
}

fn main() -> () {
    let item: GroceryItem = GroceryItem {quantity: 12, id: 1};
    show_quantity(&item);
    show_id(&item);
    return
}
