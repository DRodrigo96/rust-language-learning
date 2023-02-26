

struct Temperature {
    f_degress: f64,
}

impl Temperature {
    
    fn freezing() -> Self {
        Self { f_degress: 32.1 }
    }
    
    fn boiling() -> Self {
        Self { f_degress: 210.2 }
    }
    
    fn norm_show_f_temp(temp: &Temperature) -> () {
        println!("F temperature: {:?}", temp.f_degress);
    }
    fn self_show_f_temp(&self) -> () {
        println!("F temperature: {:?}", self.f_degress);
    }
}

pub fn funcionality() -> () {
    let current_f_temp: Temperature = Temperature {f_degress: 99.9};
    
    // Standard implementation (standard function)
    Temperature::norm_show_f_temp(&current_f_temp);
    
    // "self" implementation (lowercase)
    current_f_temp.self_show_f_temp();
    
    // "Self" implementation (capital letter)
    let cold: Temperature = Temperature::freezing();
    println!("freezing F temp: {:?}", cold.f_degress);
    cold.self_show_f_temp();
    
    let boiling: Temperature = Temperature::boiling();
    boiling.self_show_f_temp();
    boiling.self_show_f_temp();
    return
}
