use std::{collections::HashMap, thread, time::Duration};

fn main() {
    generate_worker(1, 4);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_worker(intensity: u32, random_number: u32) {
    // 閉包
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculate slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        return;
    }

    if random_number != 3 {
        println!(
            "Today, run for {} minutes",
            expensive_closure.value(intensity)
        );
        return;
    }

    println!("Done");
}
