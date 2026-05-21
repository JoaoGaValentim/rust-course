fn main() {
    // Uma função é uma sequência de passos,
    // esses são executados em ordem.
    // Tem a capacidade de armazenar uma quantidade
    // de coleções de instruções reutilizáveis.
    // Você define ela uma única vez, podendo então,
    // executa-la várias vezes.
    // As palavras "invoke" e "call", significam a mesma coisa:
    // invocar/chamar uma função.
    // Para declarar uma função usamos a palavra-chave
    // *fn*, seguida de seu nome, usando o padrão snake_case,
    // com () para params e {}.
    start_game();
    add_player_life();
    add_player_life();
    add_player_life();
    add_player_life();
    add_player_life();
    load_supreme();
    end_game();
    // Funções com parâmetros
    // Parâmetros são nomes esperados para um
    // valor de entrada de uma função.
    // São dados que fluem para dentro dela,
    // quando invocada.
    // Esses valores passados para dentro da função,
    // devem ser obrigatórios, concretos.
    // Um *argument* é um valor concreto passado
    // para a função quando ela é invocada.
    calculate_birth_year(25);
    load_people_info_card("João", 25);
    load_people_info_card("Clara", 29);
    load_people_info_card("Luna", 2);

    // Usando return
    let result: f64 = square(3.0);
    println!("square(3) = {result}");

    let result: f64 = square(3.1415);
    println!("square(3.1415) = {result}");

    let triangle_area_result: f64 = triangle_area(10.2, 33.2);
    println!("The triangle area is {triangle_area_result}.");

    // Unit
    // Unit é uma tipo de tupla vazia, ou
    // seja, sem valores.
    let result = question();
    println!("{result:#?}");

    // Blocos em funções
    let result: i32 = {
        let calcule = 21;
        let calcule = calcule * 2;
        calcule * 2
    };

    println!("{result}");
}

// Exemplos em parâmetros
fn start_game() {
    println!("Game is started!");
}

fn add_player_life() {
    println!("Added +1 life to player");
}

fn load_supreme() {
    println!("Supreme active! Go Hero!");
}

fn end_game() {
    println!("Game finished!");
}

// Exemplos com parâmetros
fn calculate_birth_year(age: u32) {
    let birth_year: u32 = 2026 - age;
    println!("Your birth year is {birth_year}.");
}

fn load_people_info_card(user_name: &str, user_age: u32) {
    println!("<div class=\"card\">");
    println!(" <h2>Name: {user_name}</h2>");
    println!(" <p>Age: {user_age}</p>");
    println!("</div>");
}

// Funções com return
// Valor de retorno é a saída de uma função.
fn square(number: f64) -> f64 {
    return number * number;
}

// Rust pode retornar o resultado da última linha
// avaliada.
fn triangle_area(base: f64, height: f64) -> f64 {
    (base * height) / 2.0
}

fn question() -> () {}
