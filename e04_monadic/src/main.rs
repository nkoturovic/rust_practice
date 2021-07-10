trait SafeOps: Sized {
    fn safe_div(self, other: Self) -> Result<Self, String>;
}

impl SafeOps for i32 {
    fn safe_div(self, other: Self) -> Result<Self, String> {
        if other == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(self / other)
        }
    }
}

fn calc() -> Result<i32, String> {
    8.safe_div(2)?.safe_div(0)?.safe_div(1)
}

fn main() {
    println!("{:?}", 2.safe_div(2));
    match calc() {
        Err(msg) => println!("Err occured: {}", msg),
        Ok(result) => println!("Result is: {}", result),
    }
}
