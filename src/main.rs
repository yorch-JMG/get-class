use std::fmt;
use chrono::{Datelike, Timelike};

use crate::get_next_class::get_next_class;
mod classes;
mod class;
mod get_next_class;

fn main() {
    impl fmt::Display for class::Day {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Day: {}, Duration: {} - {}", self.name, self.begins, self.ends)
        }
    }
    impl fmt::Display for class::Class {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Name: {}, Zoom code: {}, Password: {}", self.name, self.class_code, self.class_password)
        } 
    }

    let _hour = chrono::Local::now().hour() as i32;
    let _two_hour_ahead = chrono::Local::now().hour() as i32 + 2;
    let _minutes = chrono::Local::now().minute() as i32;
    let _day = chrono::Local::now().date().weekday().to_string();
    let _classes = classes::get_all_classes();
    let mut _counter = 0;

    _classes.iter().enumerate().for_each(|(_i, x)| {
        x.schedule.iter().enumerate().for_each(|(_j, y)| {
            if y.name.eq(&_day){
                if y.begins <= _hour && y.ends > _hour {
                    println!("{}", x);
                    println!("{}", y);
                    println!("{}", "");
                }
            }            
        });
    });
    get_next_class(_classes, _hour, _minutes);

    


}
