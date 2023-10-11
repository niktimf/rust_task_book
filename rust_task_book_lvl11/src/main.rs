

/// Уровень 1.1 задачника Rust
/// https://code.mu/ru/rust/tasker/stager/1/1/


fn main() {


    // 1. Дано число, проверьте, отрицательное оно или неn. Выведите об этом информацию в консоль.
    let num1: i32 = 123;
    match num1.is_positive() {
        true => println!("{} positive", num1),
        _ => println!("{} negative", num1),
    }


    // 2. Дана строка, выведите в консоль длину этой строки.
    let txt2: &str = "abcde";
    println!("The length of {} is {}", txt2, txt2.len());


    // 3. Дана строка, выведите в консоль последний символ строки.
    let txt3: &str = "abcde";
    println!("The last character of {} is {}", txt3, txt3.chars().last().unwrap());


    // 4. Даны 2 слова, проверьте, что первые буквы этих слов совпадают.
    let word41: &str = "abc";
    let word42: &str = "ade";
    match word41.chars().next().unwrap().eq(&word42.chars().next().unwrap()) {
        true => println!("The first letters of {} and {} are the same", word41, word42),
        false => println!("The first letters of {} and {} are different", word41, word42),
    }


    // 5. Дано слово, получите его последнюю букву. Если слово заканчивается на мягкий знак, то получите предпоследнюю букву.
    let word5: &str = "день";
    match word5.chars().last().unwrap() {
        //'ь' => println!("The last letter of {} is {}", word5, word5.chars().rev().nth(1).unwrap()),
        'ь' => println!("The last letter of {} is {}", word5, word5.chars().nth(word5.chars().count() - 2).unwrap()),
        _   => println!("The last letter of {} is {}", word5, word5.chars().last().unwrap()),
    }


    // 6. Дано число, преобразуйте его в строку.
    let num6: u16 = 123;
    println!("The string representation of {} is {}", num6, num6.to_string());


    // 7. Дано число, проверьте, что это число находится в диапазоне от 1 до 10.
    let num7: i16 = 5;
    let range = 1..=10;
    match range.contains(&num7) {
        true  => println!("{} is between 1 and 10", num7),
        false => println!("{} is not between 1 and 10", num7),
    }


}
