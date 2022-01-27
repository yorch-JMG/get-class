
pub struct Day {
    pub name: String,
    pub begins: i32,
    pub ends: i32
}

pub struct Class {
    pub name: String,
    pub class_code: i64,
    pub class_password: i32,
    pub schedule: Vec<Day>
}
