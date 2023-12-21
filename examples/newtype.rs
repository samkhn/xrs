// aliasing or naming Years/Days to i64 gives you better type checks
// (compared to just aliasing with i64 and making sure the programmer behaves well)

struct Years(i64);
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

struct Days(i64);
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_in_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_in_days.to_years()));
    // The following won't work:
    // println!("Old enough {}", old_enough(&age_in_days));
}
