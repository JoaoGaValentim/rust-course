fn main() {
    // loop
    // Refere-se a uma sequência que se repete
    // várias e várias vezes.
    // Podendo ter uma condição de parada.
    let student_tests: [f64; 6] = [10.0, 8.2, 9.3, 10.0, 10.0, 9.0];
    let mut total: f64 = 0.0;
    let mut index: usize = 0;
    let mut countdown: usize = student_tests.len();

    loop {
        if countdown == 0 {
            break; // interrompe o loop
        }

        total += student_tests[index];

        index += 1;
        countdown -= 1;
    }

    let average: f64 = total / student_tests.len() as f64;
    println!("Student average is {average:.2}.");

    // continue
    // Força o loop ir para
    // o início da próxima iteração.
    // Permite que o loop continue,
    // diferente do break, que finaliza ele.
    let mut sum: i32 = 0;

    for i in 0..=10 {
        if i % 2 == 1 {
            continue;
        }

        sum += i;
    }

    println!("The even sum is {sum}");

    // com loop
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut total: i32 = 0;
    let mut index: usize = 0;

    // loop {
    //     if index == numbers.len() {
    //         break println!("EVEN_SUM={total}");
    //     }

    //     if numbers[index] % 2 == 1 {
    //         // sem ela termos loop infinito
    //         index += 1; // condição de continuidade
    //         continue;
    //     }

    //     println!("numbers[{index}] = {}", numbers[index]);
    //     total += numbers[index];

    //     index += 1;
    // }

    // While continua repitindo um bloco de código
    // até que uma condição não for entendida.
    // Em outras palavras, termina quando uma condição
    // deixar de ser true.
    while index != numbers.len() {
        if numbers[index] % 2 == 1 {
            continue;
        }

        total += numbers[index];
        index += 1;
    }

    println!("EVEN_SUM={total}");
}
