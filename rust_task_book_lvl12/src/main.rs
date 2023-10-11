

/// Уровень 1.2 задачника Rust
/// https://code.mu/ru/rust/tasker/stager/1/2/


fn main() {


    // 1. Расскажите, в чем отличие типа i8 от типа u8.
    // i8 целочисленный тип, со знаком, длинной 8 бит(-128..127)
    // u8 целочисленный тип, без знака, длинной 8 бит(0..255)


    // 2. Дано целое число, выведите в консоль последнюю цифру этого числа.
    let num2: i32 = 123;
    println!("Last digit of {} is {}", num2, num2 % 10);
    println!("Last digit of {} is {}", num2, num2.to_string().chars().last().unwrap().to_digit(10).unwrap());


    // 3. Дано целое число, выведите в консоль первую цифру этого числа.
    let num3: i32 = 123;
    println!("First digit of {} is {}", num3, num3 / 100);
    println!("First digit of {} is {}", num3, num3.to_string().chars().next().unwrap().to_digit(10).unwrap());


    // 4. Дано целое число, выведите в консоль сумму первой и последней цифры этого числа.
    let num4: i32 = 123;
    let first_digit = num4.to_string().chars().next().unwrap().to_digit(10).unwrap();
    let last_digit = num4.to_string().chars().last().unwrap().to_digit(10).unwrap();
    println!("Sum of first and last digit of {} is {}", num4, (num4 / 100) + (num4 % 10));
    println!("Sum of first and last digit of {} is {}", num4, first_digit + last_digit);


    // 5. Дано целое число, выведите в консоль количество цифр этого числа.
    let num5: i32 = 123;
    println!("Number of digits of {} is {}", num5, num5.to_string().len());
    println!("Number of digits of {} is {}", num5, num5.to_string().chars().count());


    // 6. Дано целое число, проверьте, четное оно или нет.
    let num6: i32 = 123;
    match num6 % 2 == 0 {
        true => println!("{} is even", num6),
        false => println!("{} is odd", num6),
    }


    // 7. Дано целое число, проверьте, что это число двухзначное.
    let num7: u8 = 10;
    let range = 10..=99;
    match range.contains(&num7) {
        true => println!("{} is two-digit", num7),
        false => println!("{} is not two-digit", num7),
    }


}
