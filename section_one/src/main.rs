

mod lessons;

fn main() {
    
    println!("\nHello, world!");
    
    // variables
    println!("\nVARIABLES");
    lessons::variables::variables();
    
    // functions
    println!("\nFUNCTIONS");
    println!("{:?}", lessons::functions::add());
    
    // println_macro
    println!("\nTHE 'println' MACRO");
    lessons::println_macro::formatting();
    
    // control_flow
    println!("\nCONTROL FLOW");
    lessons::control_flow::flowing();
    
    // repetition
    println!("\nREPETITION");
    lessons::repetition::standard_loop();
    lessons::repetition::while_loop();
    
    // arithmetic
    println!("\nARITHMETIC");
    lessons::arithmetic::operations();
    
}
