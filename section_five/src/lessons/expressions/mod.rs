

enum Access {
    _Admin,
    _Manager,
    _User,
    Guest,
}

pub fn expressions() -> () {
    let access_level: Access = Access::Guest;
    let can_access_file: bool = match access_level {
        Access::_Admin => true,
        _             => false
    };
    println!("can access: {:?}", can_access_file);
}
