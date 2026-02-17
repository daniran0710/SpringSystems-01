fn is_even(n: i32)-> bool {
    n % 2 ==0
}

fn main() {
    let numbers = [12,7,15,20,3,8,30,11,5,18];

    println!("The numbers are separated as:\n");

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} -> FizzBuzz", num);
        }
        else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        }
        else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        }
        else if is_even(num) {
            println!("{} -> Even", num);
        }
        else {
            println!("{} -> Odd", num);
        }
    }

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("\nThe sum of all numbers is: {}", sum);

    let mut largest = numbers[0];
    let mut i = 1;

    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }

        i += 1;
    }

    println!("The largest number is: {}", largest);
}