

/// Уровень 1.3 задачника Rust
/// https://code.mu/ru/rust/tasker/stager/1/3/


fn main() {


    // 1. Расскажите, сколько байт занимает тип i8.
    // В 1 байте 8 бит. Тип i8 занимает 8 бит.
    println!("Size of i8 in bytes: {}", std::mem::size_of::<i8>());


    // 2. Выведите в консоль все целые числа от 1 до 10.
    println!("Numbers from 1 to 10: ");
    (1..=10).for_each(|x| println!("{}", x));


    // 3. Дана строка. Если в этой строке более одного символа, выведите в консоль предпоследний символ этой строки.
    fn print_pre_last_symbol(s: &str) {
        match s.chars().count() {
            0 | 1 => println!("The string: {} is too short", s),
            _ => {
                match s.chars().nth(s.chars().count() - 2) {
                    Some(c) => println!("Pre last symbol of {} is {}", s, c),
                    None => println!("Unable to find pre last symbol of {}", s),
                }
            }
        }
    }

    let empty_string = "";
    print_pre_last_symbol(&empty_string);

    let one_symbol = "a";
    print_pre_last_symbol(&one_symbol);

    let two_symbols = "ab";
    print_pre_last_symbol(&two_symbols);


    // 4. Дано целое число. Проверьте, что первая и последняя цифры этого числа совпадают.
    fn is_same_first_last_digit(x: u32, y: u32) {
        let first_digit =  match x.to_string().chars().next() {
            Some (c) => c.to_digit(10),
            None => {
                println!("Unable to find first digit of {:?}", x);
                None
            }
        };

        let last_digit =  match y.to_string().chars().last() {
            Some (c) => c.to_digit(10),
            None => {
                println!("Unable to find last digit of {:?}", y);
                None
            }
        };

        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                match first.eq(&last) {
                    true => println!("{:?} and {:?} are the same", first, last),
                    false => println!("{:?} and {:?} are different", first, last),
                }
            },
            _ => println!("Unable to find first or last digit"),
        }
    }

    is_same_first_last_digit(1234, 1234);
    is_same_first_last_digit(1234, 1);
    is_same_first_last_digit(1, 1236);


    // 5. Даны два целое числа. выведите в консоль наибольшее из них.
    fn max_of_two(x: u16, y: u16) {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => println!("Max of {:?} and {:?} is {:?}", x, y, y),
            std::cmp::Ordering::Greater => println!("Max of {:?} and {:?} is {:?}", x, y, x),
            std::cmp::Ordering::Equal => println!("{:?} and {:?} are equal", x, y),
        }
    }
    let num51: u16 = 36;
    let num52: u16 = 12;
    max_of_two(num51, num52);


    // 6. Дана строка, содержащая целое число, преобразуйте эту строку в число.
    let txt6: &str = "123";
    println!("{}", txt6.parse::<u32>().unwrap());


    // 7. Дано целое число, содержащее номер месяца от 1 до 12, определите, в какую пору года попадает этот месяц.
    let num7: u8 = 1;
    match num7 {
        1 => println!("January"),
        2 => println!("February"),
        3 => println!("March"),
        4 => println!("April"),
        5 => println!("May"),
        6 => println!("June"),
        7 => println!("July"),
        8 => println!("August"),
        9 => println!("September"),
        10 => println!("October"),
        11 => println!("November"),
        12 => println!("December"),
        _ => println!("Invalid month"),
    }


}
