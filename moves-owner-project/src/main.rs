fn main() {
    // let time = 2026;
    // // this is a full copy of the 'time'
    // let year = time;

    // Não estamos criando uma cópia aqui,
    // pois, esse tipo não implementa a trait Copy.
    let person: String = String::from("João");

    // Quando colocamos o valor da variável 'person'
    // em genius, estamos movendo o dono,
    // após a linha 13 ela deixa de existir.
    let genius: String = person;

    // drop é uma função padrão do Rust
    // drop(genius) -> invalida e deleta a referência,
    // deixando de existir.
    println!("{genius}");

    // Clone
    // clona o valor de um owner canônico,
    // gerando o original e em outro owner, como
    // uma cópia.

    // se eu multiplicar esse por 2, só aqui vai valer
    let prime_universe = String::from("137");
    // se eu multiplicar esse por 3 só aqui vai valer,
    // dado que é um clone.
    let ricks_universe = prime_universe.clone();
    println!("The Rick's Prime universe is {prime_universe}");
    println!("The Rick's Sanchez universe is C-{ricks_universe}");
}
