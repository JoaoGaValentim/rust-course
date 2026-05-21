#![allow(unused)]
type Price = f64;

fn main() {
    app();

    let mut seconds: i32 = 59;

    // a palavra rezervada loop
    // executará indefinidamente
    // se não houver instrução de parada
    loop {
        if seconds == 0 {
            break println!("Go!!!!");
        }

        println!("{seconds}");
        seconds -= 1;
    }
}

fn app() {
    // Fluxo de controle (control flow), refere-se a,
    // como um programa será executado.
    // O poder da programação vem das condições, ou seja,
    // decisões que ele toma para alcançar um objetivo, se e somente se
    // uma condição for atendida.
    let age: i32 = 18;

    // if
    // Rust exige que a expressão seja
    // boolean. (true ou false)
    if age >= 18 {
        println!("The people is adult.");
    }
    println!("And or age is {age} years old.");

    let age: i32 = 10;
    // se for falso não é executado
    if age < 18 {
        println!("This people is a child or young.");
    }
    println!("And or age is {age} years old.");

    let http_status_code: i32 = 200;
    let http_status_code: i32 = http_status_code + 200;
    let http_status_code: i32 = http_status_code + 100;
    let http_status_code: i32 = http_status_code + 100;
    let http_status_code: i32 = http_status_code - 500;

    // else if
    // verifica um cadeia de expressões
    // até constatar e validar se a expressão é verdadeira.
    if http_status_code >= 200 && http_status_code < 300 {
        println!("HTTP success record.");
    } else if http_status_code >= 300 && http_status_code < 400 {
        println!("HTTP redirect record.");
    } else if http_status_code >= 400 && http_status_code < 500 {
        println!("HTTP validation record.");
    } else if http_status_code >= 500 {
        println!("HTTP Server error record.");
    } else {
        println!("HTTP invalid status code.");
    }

    // if/else
    let user_names: [&str; 4] = ["Clara", "Ana", "Lucas", "Pedro"];

    if user_names.contains(&"Clara") {
        println!("users_name contains Clara");
    } else {
        println!("users_name don't contains Clara :<");
    }

    println!("{}", even_or_odd(17));

    // Match
    // O match funciona como um caso de switch,
    // como em outras linguagens de programação.
    // Ela permite que possamos reagir a vários
    // variantes possíveis de um valor
    let evaluation: bool = true;

    let evaluation: i32 = match evaluation {
        // um pattern ou arm (braço)
        // representa um valor para a
        // comparação da avaliação inicial.
        true => 20,
        false => 40,
    };

    println!("{evaluation}");

    let price: Price = 4431.22;
    let price: Price = calculate_discount_price(price);

    println!("The price with dicount as ${price:.2}");

    let value: i32 = 10;

    match value {
        2 | 4 | 6 | 8 | 10 => println!("Valid :)"),
        _ => println!("Invalid :<"),
    }

    let number: i32 = 12;

    match number {
        value if value % 2 == 0 => println!("{value} is Even."),
        value if value % 2 != 0 => println!("{value} is Odd."),
        _ => unreachable!(),
    }

    let list_values: [i32; 100] = generate_list_start_with(2000);
    println!("result: {:?}", list_values);
}

fn calculate_discount_price(price: Price) -> Price {
    let discount: Price = match price {
        4000.0..=6000.99 => 0.5,
        1000.0..=3999.99 => 0.2,
        // _ (underline) é chamado de padrão catch-all, coringa  para simplificar.
        // Compreenderá qualquer valor possível, caso os outros braços,
        // não tenham uma combinação esperada.
        // É equivalente a uma instrução else
        // Ele deve sempre ser o ultimo, pois se for o primeiro invalida os outros.
        _ => 0.1,
    };

    price - (price * discount)
}

fn even_or_odd(number: i32) -> String {
    let message: String = if number % 2 == 0 {
        String::from("Number is even.")
    } else {
        String::from("Number is odd.")
    };

    message
}

fn generate_list_start_with(start: i32) -> [i32; 100] {
    // iterar - fazer várias e várias vezes algo
    let mut count: usize = 0;
    let mut numbers: [i32; 100] = [0; 100];

    let list_values: [i32; 100] = loop {
        numbers[count] = count as i32 + start;
        count += 1;

        if count == 100 {
            break numbers;
        }
    };

    list_values
}
