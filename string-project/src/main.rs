#![allow(unused)]

fn remove_assents(src: &str) -> String {
    if src.is_empty() {
        return String::new();
    }

    let mut text: String = String::from(src);

    for letter in src.chars() {
        let str_letter: &str = &letter.to_string();
        match str_letter {
            "ã" | "á" | "â" => text = text.replace(&str_letter, "a"),
            "õ" | "ô" | "ó" | "ó" => text = text.replace(&str_letter, "o"),
            "é" | "ê" => text = text.replace(&str_letter, "e"),
            _ => {}
        };
    }

    text
}

fn len(src: &str) -> usize {
    remove_assents(src).len()
}

fn main() {
    // let name = "João";

    // String é um tipo complexo
    // pode ser trabalhado com a memória Heap.
    // Type::function (usamos para acessar algo em um namespace )
    // let text: String = String::from("Hello, World!");
    let mut name: &str = "é";

    println!("{} {}", len(name), remove_assents(name));
}
