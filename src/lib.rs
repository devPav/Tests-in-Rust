#[derive(Debug, PartialEq)]
pub struct Rectangle {
    lenght: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.lenght > other.lenght && self.width > other.width
    }
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Value must be more or equal than 1, we took {}", value);
        } else if value < 1 {
            panic!("Value must be less or equal than 100, we took {}", value);
        };
        Guess { value }
    }
}
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", "name")
}
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I received value {}", a);
    10
}
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            lenght: 8,
            width: 7,
        };
        let smaller = Rectangle {
            lenght: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            lenght: 8,
            width: 7,
        };
        let smaller = Rectangle {
            lenght: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(3, add_two(2));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("John");
        assert!(
            result.contains("John"),
            "Greeting does not contain name, value was provided '{}'",
            result
        );
    }
    #[test]
    #[should_panic(expected = "Value must be less or equal than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two is not four"))
        }
    }
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(5, value);
    }
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}