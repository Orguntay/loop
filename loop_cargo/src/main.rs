fn main() {
    let mut counter = 0;
    'counter_up: loop {
        println!("counter = {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counter_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End counter = {}", counter);
    lp();
    while_loop();
    array_loop();
    for_loop();
    rev_rev();
    fibonacci(30, 1, 1);
    println!("Verilen son sayıya kadar olan fibonacci sayıları TAMAMDIR!");
}

fn lp() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Sonuç {}", result);
}

fn while_loop() {
    let mut sayi = 0;
    while sayi != 0 {
        println!("{}!", sayi);
        sayi -= 1;
    }
    println!("Eski bir yazılımcının da dediği gibi TAMAMDIR.");
}

fn array_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("Değer {}", a[index]);
        index += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The valoe is: {}", element);
    }
}

fn rev_rev() {
    for element in (1..4).rev() {
        println!("{}!", element);
    }
    println!("TAMAMDIR!");
}

fn fibonacci(end_number: i32, fibo_a: i32, fibo_b: i32) {
    if fibo_a < end_number {
        let fibo = fibo_a + fibo_b;
        println!("Fibonacci sayısı: {}", fibo);
        fibonacci(end_number, fibo_b, fibo);
    }
}
