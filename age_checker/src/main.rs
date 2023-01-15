// newtype example - using a tuple struct to wrap a type so we can add custom methods to it
// and enforce type checks, while still retaining the original type's features.

use std::ops::Deref;
use std::default::Default;

// Both of these types derive from i64, but they are distinct and not interchangable

#[derive(Default)]
struct Years(i64);

#[derive(Default)]
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 *   356)
    }
}

impl Deref for Years {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

impl Deref for Days {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 > 21
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_days.to_years()));
    println!("Age in days: {}", *age_days);
    let new_age: Years = Default::default();
    println!("Default age in years: {}", *new_age);

    // This won't work:
    // println!("Old enough: {}", old_enough(&age_days));
}
