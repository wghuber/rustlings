// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM NOT DONE

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn print_numbers(maybe_numbers: &[Option<u16>; 5]) {
    for iter in 0..5 {
        let maybe_number = maybe_numbers[iter as usize];

        if let Some(number) = maybe_number {
            println!("printing: {}", number);
        }
        if maybe_number.is_some() {
            println!("printing: {}", maybe_number.unwrap());
        }
    }
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = Default::default();
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 1235) + 2) / (4 * 16) };

        if let None = numbers[iter as usize] {
            numbers[iter as usize] = Some(number_to_add);
        }
    }
    print_numbers(&numbers);
}
