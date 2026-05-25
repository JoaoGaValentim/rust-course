#![allow(unused)]
fn main() {
    // Espoco de um programa é formado
    // por blocos ({}).

    let age: i32 = 25; // idade vive em main();
    let bith_year: i32 = 2026 - age;
    println!("{bith_year}");

    {
        let is_legal: bool = true; // só existe aqui
    } // não existe aqui
}
