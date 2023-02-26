

#[derive(Debug)]
enum BoxColor {
    _Black,
    _White,
    Red,
}

struct ShippingBox {
    dimension: f64,
    weight: f64,
    color: BoxColor
}

impl ShippingBox {
    
    fn box_model(dimension: f64, weight: f64, color: BoxColor) -> Self {
        Self { dimension, weight, color }
    }
    
    fn show_charac(&self) -> () {
        println!("Dimension: {:?}", self.dimension);
        println!("Weight: {:?}", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() -> () {
    let (dimension_, weight_, color_): (f64, f64, BoxColor) = (237.12, 21.52, BoxColor::Red);
    let shipping_box: ShippingBox = ShippingBox::box_model(dimension_, weight_, color_);
    shipping_box.show_charac();
    return
}
