#![allow(unused)]
fn main() {
    // Borrowing significa empréstimo
    // Usamos esse valor como referência
    // Prometemos devolver ele depois
    // que usar-mos.
    // A referência tem um custo muito menor
    // que a cópia, pois encaminha-nos, para
    // o endereço que possui o valor de dada vari-
    // ável.
    // As Regras de Referência
    // Vamos recapitular o que discutimos sobre referências:
    // Em qualquer momento, você pode ter uma única referência mutável
    // ou qualquer número de referências imutáveis.
    // As referências devem sempre ser válidas.
    // & (endereço que leva á...) -> operador de empréstimo
    let name = String::from("joão");
    let name_upper = upper(&name);

    println!("{name} {name_upper}");

    let x = 12;
    let y = 14;
    let z = 10;
    let w = &y;
    let v = &z;
    let u = &z;
    let mut sum = w + u + v;
    sum *= v * v * w * w * u * u;
    println!("{x} {y} {z} {w} {u} {v} {sum}");
}

fn upper(value: &String) -> String {
    value.to_uppercase()
}
