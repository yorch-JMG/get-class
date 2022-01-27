use std::fmt;
use chrono::{Timelike, Datelike};
mod classes;
mod class;
fn main() {
    impl fmt::Display for class::Day {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Day: {}, Begins: {}, Ends: {}", self.name, self.begins, self.ends)
        }
    }
    impl fmt::Display for class::Class {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Name: {}, Zoom code: {}, Password: {}", self.name, self.class_code, self.class_password)
        } 
    }

    let _hour = chrono::Local::now().hour() as i32;
    let _minutes = chrono::Local::now().minute() as i32;
    let _day = chrono::Local::now().date().weekday().to_string();
    let _classes = classes::get_all_classes();

    _classes.iter().enumerate().for_each(|(_i, x)| {
        x.schedule.iter().enumerate().for_each(|(_i, y)| {
            if y.name.eq(&_day) && y.begins <= _hour && y.ends > _hour {
                println!("{}", x);
                println!("{}", y);
                if _minutes > 30 && _i+1 < _classes.len() {
                    let _next_class = &_classes[_i+1];
                    println!("{}", _next_class);
                    println!("Begins: {}", _next_class.schedule[0]);
                    println!("Ends: {}", _next_class.schedule[1]);
                }
            }
        });
    });
    


}