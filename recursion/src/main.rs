#![allow(unused)]
fn main() {
    // Podemos debbugar para encontrar erros
    // em nosso código e fixa-los.
    // Temos o conceito de ponto de parada, ou
    // breakpoint, para interromper o código em um
    // dado ponto.
    // Com isso podemos verificar os valores
    // de variáveis, execuções de funções e
    // etc.
    // println!("{}", fibonacci(15));
    // println!("{}", factorial(5));
    // println!("============");
    // countdown(10, 1);
    // println!("============");
    // countdown(1, 11);
    // println!("============");
    generate_multiplication_table(0, 10, 3);
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
    println!("{start}");
    if start == end {
        return;
    }

    if start > end {
        countdown(start - 1, end);
    }

    if end > start {
        countdown(start + 1, end);
    }
}

fn generate_multiplication_table(start: i32, end: i32, number: i32) {
    let value: i32 = start * number;

    println!("{start} x {number} = {value}");

    if start == end {
        return;
    }

    if start > end {
        generate_multiplication_table(start - 1, end, number);
    }

    if end > start {
        generate_multiplication_table(start + 1, end, number);
    }
}
