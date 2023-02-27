

#[derive(Debug)]
struct Person {
    age: u32,
    name: String,
    color: String,
}

impl Person {
    fn build(age: u32, name: String, color: String) -> Self {
        Self { age, name, color }
    }
}

fn show_info(person: &Person) -> () {
    println!("name: {:?}", person.name);
    println!("favorite color: {:?}", person.color);
}

fn main() {
    let people: Vec<Person> = vec![
        Person::build(08, String::from("aaa"), String::from("xxx")),
        Person::build(15, String::from("bbb"), String::from("yyy")),
        Person::build(10, String::from("ccc"), String::from("zzz")),
    ];
    for person in &people {
        if person.age <= 10 {
            show_info(person)
        }
    };
    println!("{:?}", people);
    return
}
