use crate::class::Class;
use chrono::{Datelike, Timelike};

pub fn get_next_class(_classes: Vec<Class>, _current_hour: i32, _current_minute:i32){
    let _two_hour_ahead = chrono::Local::now().hour() as i32 + 2;
    let _hour = _current_hour + 2; 
    let _day = chrono::Local::now().date().weekday().to_string();

    _classes.iter().enumerate().for_each(|(_i, x)| {
        x.schedule.iter().enumerate().for_each(|(_j, y)| {
            if y.name.eq(&_day) && y.ends >= _hour && 30 > _current_minute {
                let mut _next_class = x;
                    println!("{}", "Next up:");
                    println!("{}", x);
                    println!("{}", y);
                    println!("{}", "");
            }            
        });
    });
    
}
