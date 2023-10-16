

/// Уровень 1.5 задачника Rust
/// https://code.mu/ru/rust/tasker/stager/1/5/

fn main() {


    // 1. Расскажите, какой диапазон значений может принимать тип i16.
    // Тип i16, хранит целые числа со знаком длиной 16 бит, принимает значения от -32768 до 32767.


    // 2. Выведите в консоль все числа кратные 7 в промежутке от -100 до 100.
    let start = -100;
    let end = 100;
    match (start..=end).start() % 7 == 0 {
        true => (start..=end).step_by(7).for_each(|x| println!("{}", x)),
        false => {
            match (start..=end).find(|&x| x % 7 == 0) {
                Some(x) => {
                    (x..=end).step_by(7).for_each(|x| println!("{}", x))
                }
                None => {
                    println!("No numbers are divisible by 7");
                }
            }
        }
    }


    // 3. Даны два целых числа, проверьте, что первые цифры этих чисел совпадают.
    let num31: u16 = 12;
    let num32: u16 = 13;
    match num31.to_string().chars().nth(0).unwrap().cmp(&num32.to_string().chars().nth(0).unwrap()) {
        std::cmp::Ordering::Equal => println!("The first numbers are equal"),
        _ => println!("The first numbers are not equal"),
    }


    // 4. Даны два целых числа, проверьте, что первое число без остатка делится на второе.
    let num41: u16 = 36;
    let num42: u16 = 12;
    match num41 % num42 == 0 {
        true => println!("The first number is divisible by the second without remainder"),
        false => println!("The first number is not divisible by the second without remainder"),
    }


    // 5. Дана некоторая строка, переберите и выведите в консоль по очереди все символы с конца строки.
    let txt5: &str = "123456789";
    txt5.chars().rev().for_each(|x| println!("{}", x));


    // 6. Даны три целых числа, выведите в консоль большее из этих чисел.
    let num61: u16 = 36;
    let num62: u16 = 24;
    let num63: u16 = 12;
    println!("The largest number is {}", std::cmp::max(num61, std::cmp::max(num62, num63)));


    // 7. Даны три символа, слейте эти символы в строку.
    let chr71: char = 'a';
    let chr72: char = 'b';
    let chr73: char = 'c';

    let str7_1: String = format!("{}{}{}", chr71, chr72, chr73);
    println!("{}", str7_1);

    let str7_2: String = chr71.to_string() + &chr72.to_string() + &chr73.to_string();
    println!("{}", str7_2);


    // 8. Дано целое число, содержащее количество килобайт, переведите это значение в байты.
    let kb: u32 = 35;
    println!("{} bytes", kb * 1024);


    // 9. Дано целое число, содержащее номер дня от 1 до 31, определите, в какую декаду месяца попадает этот день.
    let num: u16 = 1;
    match num {
        1..=10 => println!("First decade"),
        11..=20 => println!("Second decade"),
        21..=31 => println!("Third decade"),
        _ => println!("Invalid day"),
    }


}