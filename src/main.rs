use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value, simulated_random_number
    );
}

struct Cacher<T,In,Out> 
    where T: Fn(In)->Out 
{
    calculation: T,
    results: HashMap<In, Out>,
}


impl<T,In,Out> Cacher<T,In,Out>
    where T: Fn(In)->Out,
          In: Eq + std::hash::Hash + Copy,
          Out: Eq + std::hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T,In,Out> {
        Cacher {
            calculation,
            results: HashMap::new(),
        }
    }

    fn value(&mut self, arg: In) -> Out {

        match self.results.get(&arg) {
            Some(v) => *v,
            None => {
                self.results.insert(arg, (self.calculation)(arg));
                self.value(arg)
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today do {} pushups",
            expensive_result.value(&intensity)
        );

        println!(
            "Then do {} situps",
            expensive_result.value(&intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hydrated!")
        } else {
            println!(
                "Today run for {} minutes.",
                expensive_result.value(&intensity) 
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

