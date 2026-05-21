#![allow(unused)]
fn main() {
    // Podemos debbugar para encontrar erros
    // em nosso código e fixa-los.
    println!("{}", fibonacci(15));
    println!("{}", factorial(5));
    println!("============");
    countdown(10, 1);
    println!("============");
    countdown(1, 11);
    println!("============");
}

// Recursão é quando uma função
// chama a si mesma.
// Todas possuem um caso base, uma condição
// de parada. Ela impede outras auto-invokes da mesma.
fn fibonacci(value: usize) -> usize {
    if value == 0 {
        return 0;
    }

    if value == 1 {
        return 1;
    }

    fibonacci(value - 1) + fibonacci(value - 2)
}

fn factorial(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }

    n * factorial(n - 1)
}

fn countdown(start: i32, end: i32) {
    if start == end {
        println!("{start}");
        return;
    }

    if start > end {
        println!("{start}");
        countdown(start - 1, end);
    }

    if end > start {
        println!("{start}");
        countdown(start + 1, end);
    }
}
