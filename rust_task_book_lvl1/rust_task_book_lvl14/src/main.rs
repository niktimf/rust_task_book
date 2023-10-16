

/// Уровень 1.4 задачника Rust
/// https://code.mu/ru/rust/tasker/stager/1/4/


fn main() {


    // 1. Какой диапазон значений может принимать тип i8.
    // Тип i8 принимает значения от -128 до 127.


    // 2. В чем отличие типа i16 от типа f32.
    // Тип i16, хранит целые числа со знаком длиной 16 бит, принимает значения от -32768 до 32767
    // Тип f32, хранит дробные числа со знаком длиной 32 бит, принимает значения от -3.4E38 до 3.4E38


    // 3. Выведите в консоль все четные числа из промежутка от 2 до 100.
    let range = 2..=100;
    match range.start() % 2 == 0 {
        true => range.step_by(2).for_each(|x| println!("{}", x)),
        false => (range.start() + 1..*range.end()).step_by(2).for_each(|x| println!("{}", x)),
    }


    // 4. Дана некоторая строка, переберите и выведите в консоль по очереди все ее символы.
    let txt4: &str = "abcde";
    txt4.chars().for_each(|x| println!("{}", x));


    // 5. Даны два целых числа получите остаток деления первого числа на второе.
    let num51: u16 = 36;
    let num52: u16 = 12;
    println!("Remainder of division is {}", num51 % num52);


    // 6. Дано целое число, содержащее номер минуты от 0 до 60.
    // Определите, в какую четверть часа попадает это значение.
    let num6: u8 = 30;
    match num6 {
        0..=15 => println!("First quarter"),
        15..=30 => println!("Second quarter"),
        30..=45 => println!("Third quarter"),
        45..=60 => println!("Fourth quarter"),
        _ => println!("Invalid quarter"),
    }


    // 7. Дано двухзначное число, проверьте, что в нем вторая цифра больше первой.
    let num7: u8 = 12;
    let num7_to_string= num7.to_string();
    match num7 {
        10..=99 => match num7_to_string.chars().nth(1).unwrap().cmp(&num7_to_string.chars().nth(0).unwrap()) {
            std::cmp::Ordering::Greater => println!("The second number is greater than the first"),
            std::cmp::Ordering::Less => println!("The second number is less than the first"),
            std::cmp::Ordering::Equal => println!("The second number is equal to the first")
        },
        _ => println!("Number is not two-digit")
    }


}
