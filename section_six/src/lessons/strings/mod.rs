

#[derive(Debug)]
struct LineItem {
    name: String,
    count: i32
}

fn show_name(name: &str) {
    println!("name: {:?}", name)
}

pub fn strings() {
    let receipt: Vec<LineItem> = vec![
        LineItem {name: "cereal".to_owned(), count: 1},
        LineItem {name: String::from("fruit"), count: 1}
    ];
    
    for re in &receipt {
        show_name(&re.name);
        println!("count: {:?}", re.count)
    };
    println!("{:?}", receipt);
    return
}
