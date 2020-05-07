use std::collections::HashMap;

//-- Closure

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
    fn new (calculation: T) -> Cacher<T> {
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
                v.clone()
            }
        }
    }
}


pub fn run_closure_ex() {
    let simulated_user_specified_value = 10;
    let simulated_rand_num = 7;

    generate_workout(simulated_user_specified_value, simulated_rand_num);
}

fn generate_workout(intensity: u32, rand_num: u32) {
    let mut expensive_calc_cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Do {} push ups!", expensive_calc_cacher.value(intensity));
        println!("Do {} sit ups!", expensive_calc_cacher.value(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break...");
        } else {
            println!{"Run {} min.", expensive_calc_cacher.value(intensity)};
        }
    }
}


//-- Iterators

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0, }
    }
}

// Implement the Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cacher_diff_values() {
        let test_closure = |num| {
            num
        };
        let mut cacher = Cacher::new(test_closure);
        let v1 = cacher.value(1);
        let v2 = cacher.value(2);

        assert_eq!(cacher.value(1), 1);
        assert_eq!(cacher.value(2), 2);
    }

    #[test]
    fn test_iterator_map() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn test_filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 9,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let shoes_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(shoes_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]);
    }

    #[test]
    fn test_counter_iterator() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_counter_iterator_to_vector() {
        let mut counter_iter = Counter::new().into_iter();
        let collected_vec: Vec<u32> = counter_iter.collect();

        assert_eq!(collected_vec, vec![1, 2, 3, 4, 5]);
    }
}
