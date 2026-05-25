fn main() {
    // Traits -> contratos
    //  -> metodos e regras a serem seguidos
    // Todos os tipos primitivos possuem a trait copy

    let time: i32 = 2026;
    // year recebe uma cópia
    // não é uma referência
    // ao valor original de 'time'.
    let year: i32 = time;

    println!("The time is {time}.");
    println!("It is the year is {year}.");
}
