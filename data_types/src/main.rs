fn main() {
    // --------- NUMERICS INTS ---------
    // Em rust temos i (para números inteiros),
    // positivos e negativos.
    // Se inferirmos o tipo, o compiler, coloca i32
    // por padrão.
    // A notação é -2⁽ⁿ⁻¹⁾ para 2⁽ⁿ⁻¹⁾ - 1

    // i8 (-128 - 127)
    let eight_bit_signed: i8 = 127;
    // i16 (-32.768 - 32.767)
    let sixteen_bits_signed: i16 = 32_767;
    // i32 (default do rust) (-2.147.483.648 - 2.147.483.647)
    let thirty_two_bits_signed: i32 = 2_147_483_647;

    println!("i8={eight_bit_signed} i16={sixteen_bits_signed} i32={thirty_two_bits_signed}");
    // Declaração alternativa dos assinalados
    let eight_bit_signed = 127i8;
    let sixteen_bits_signed = 32_767i16;
    let thirty_two_bits_signed = 2_147_483_647i32;

    println!("i8={eight_bit_signed} i16={sixteen_bits_signed} i32={thirty_two_bits_signed}");

    // Usamos u para, inteiros positivos.
    // A notação é 0 para 2⁸ - 1

    // u8 (0 - 255)
    let eight_bit_unsigned: u8 = 255;
    // u16 (0 - 65.535)
    let sixteen_bits_unsigned: u16 = 16;
    // u32 (0 - 4.294.967.295)
    let thirty_two_bits_unsigned: u32 = 32;

    println!("u8={eight_bit_unsigned} u16={sixteen_bits_unsigned} u32={thirty_two_bits_unsigned}");

    // Declaração alternativa dos não assinalados
    let eight_bit_unsigned = 255u8;
    let sixteen_bits_unsigned = 16u16;
    let thirty_two_bits_unsigned = 32u32;

    println!("u8={eight_bit_unsigned} u16={sixteen_bits_unsigned} u32={thirty_two_bits_unsigned}");

    // USIZE e ISIZE (aliases para a arquitetura em bits do SO usado)
    // Se 32 bits = i32 ou u32
    // Se 64 bits = i64 ou u64
    let days_of_first_month_of_year: usize = 31;
    let days_of_one_year: usize = 365;
    println!(
        "Day of month= {} & Days of one year= {}",
        days_of_first_month_of_year, days_of_one_year
    );

    let presence: isize = 22;
    let presence: isize = presence - 1;
    println!("The student has presence equals {presence}.");
    // ---------------------------

    // --------- NUMERICS FLOATING POINTS ---------
    // Representam números decimais com ponto, divididos
    // por casas fracionárias (ex.: PI = 3.1415), como
    // componente.
    // Existem duas unidades de float, assinaladas (-/+):
    // - f32 (float de 32 bits (-/+), 6-7 dígitos (-/+), hardware limitado)
    // - f64 (Default, float de 64 bits, 15-17 dígitos (-/+), exige mais hardware)
    // Dígitos de precisão: Definem a precisão após o ponto.
    // Não existe float unsigned, todos tem positivos e negativos.

    let pi: f64 = 3.1415926535897932384;
    // chão (menor ou igual ao valor, int)
    println!("MENOR_OU_IGUAL={}", pi.floor());
    // teto (maior ou igual ao valor, int)
    println!("MAIOR_OU_IGUAL={}", pi.ceil());
    // arrendondamento para o inteiro em 0.0
    println!("ARREDONDADO_PROXIMO={}", pi.round());

    // Formatação de Floats
    // {value:.3} ele calcula o dobro, para,
    // fazer a representação matemática correta.
    // 7 (precisão)
    println!("{pi:.7}");

    // 16 (precisão)
    println!("{pi:.16}");

    // 19 (precisão)
    println!("{pi:.19}");

    // --------------------------------------------
    // --------- STRINGS ---------
    // Coleção de caracteres em uma sequência que formam
    // um texto.

    // String literal são conhecidas em tempo de compilação
    println!("\tHello \nWorld!"); // "..." -> String literal.
    // \n (break line) & \t (tab line)
    println!("Tobias said: \"Good Irene!\""); // \" (literal)
    let file_path_on_windows: &str = r"c:\system32\"; // r (raw, trata o texto literal)
    println!("win32={file_path_on_windows}");
    // ----------------------------

    // --------- METHODS ---------
    // Instruções aplicadas a um dado valor,
    // usando notação value.method(), para aplica-lo.
    // Reside no valor.
    // Podem aceitar argumentos, para personalizar sua saída,
    // durante o runtime.
    // Todos os tipos em Rust possuem métodos, podendo,
    // customiza-los no runtime.
    // Os métodos com tipos devem ter ele declarado
    // explicitamente, principalmente numéricos.
    let name_with_empty_space: &str = " João ";
    let scale: i32 = -32;
    println!("{} and {}.", name_with_empty_space.trim(), scale.abs()); // distancia de 0 (abs) 

    let scale: i32 = -15;
    println!("{}", scale.pow(2));
    println!("{}", scale.pow(3));

    // ------ Casting de variáveis ------
    // Converte o valor da esquerda para o da direita,
    // usando a palavra reservada "as".
    // let variable = 10.2 as i32;
    // variable é inteira de 32 bits agora, ou seja, vai valer
    // 10.
    let size_in_meters: f64 = 233.83;
    let size_in_meters_integer: u8 = size_in_meters as u8;
    let size_in_meters_int_as_char: char = size_in_meters_integer as char;
    println!(
        "size_float={}, size_int_unsigned={}, char={}",
        size_in_meters, size_in_meters_integer, size_in_meters_int_as_char
    );

    // ------ Operadores matemáticos ------
    // Operadores: + (mais), - (menos), * (multiplicação), / (divisão)
    // e % (resto da divisão)
    // operando operador operando => 5 + 5 = 10
    let calculate_value: i32 = 5;
    let calculate_value: i32 = calculate_value + 5; // Adição
    let calculate_value: i32 = calculate_value - 2; // Subtração
    let calculate_value: i32 = calculate_value * 4; // Multiplicação
    // Rust realiza uma divisão de "piso" (floor) menor inteiro.
    let calculate_value: i32 = calculate_value / 12;
    let calculate_value: i32 = calculate_value % 2; // Módulo
    println!("The calcule result is equals to {calculate_value}.");

    let average: i32 = ((10.0 + 10.0 + 10.0 + 3.5) / 4.0) as i32;
    println!("The calcule of average floor is equals {average}.");

    let average: f64 = (10.0 + 10.0 + 10.0 + 3.5) / 4.0;
    println!("The calcule of average decimal is equals {average}.");

    // ------------------------------------

    // ------ Operadores de atribuição, Binary Operators ------
    // São uma maneira simplificada de realizar,
    // e atribuir um valor, podendo ser inteiro, decimal
    // ou String
    let mut year: i32 = 2025;
    year += 1;
    year *= 2;
    year /= 2;
    println!("The New Year is {year}!");
    year -= 7;
    println!("The seven years, before is {year}!");
    // ---------------------------------------------------------
    // -------------------- BOOLEANS ---------------------------
    // Termo deriva do nome do matemático inglês, George Bool. O
    // tipo boolean em Rust, assim como, em outras linguagens de
    // programação, ocupa um byte de tamanho e representa, true ou
    // false, sendo possível somente um valor possível: Verdadeiro,
    // ou senão, Falso. (0 ou 1)

    // > (menor que)
    let age: i32 = 25;
    let is_on: bool = false;
    let is_young: bool = age < 35;
    println!("The computer is on? {is_on}.");
    println!("Is young? {is_young}.");
    println!("{} {}", age.is_positive(), age.is_negative());

    // -- NOT (!) --
    // Se verdade negada, vira falso e
    // se falso negado, vira verdade.
    // |⁻⁻⁻⁻⁻⁻⁻|⁻⁻⁻⁻⁻⁻⁻|
    // | p     | !p    |
    // | ⁻⁻⁻⁻⁻ | ⁻⁻⁻⁻⁻ |
    // | true  | false |
    // | false | true  |
    // |_ _ _ _|_ _ _ _|
    println!("-----------------------------");
    println!("{0} {1}", true, !true);
    println!("{0} {1}", false, !false);
    println!("-----------------------------");

    // Nos USA, filmes são classificados em:
    // G - Filme para todas as idades
    // PG (Parental Guidance Suggested) - Acompanhado de um adulto,
    // pode conter violência breve e linguagem leve.
    // PG-13 - Acompanhado de um adulto, violência extrema e
    // temas mais adultos.
    // R - Adultos, pessoas com 17 ou mais podem assistir.
    // NC-17 - Muito brutal.
    let age: i32 = 13;
    let can_see_r_movie: bool = age >= 17;
    let cannot_see_r_movie: bool = !can_see_r_movie;
    println!("I am {age} years old. Can I see this scary movie? {cannot_see_r_movie}.");
    println!(
        "can_see={} cannot_see={}",
        can_see_r_movie, cannot_see_r_movie
    );

    // == (Igualdade) & != (Desigualdade)
    // == é usado para verificar se dois valores boolean, são:
    // verdadeiros ou falsos
    // |-------------------------------|
    // |   p    |    q   | p == q      |
    // |⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻|
    // |  true  |  true  | true        |
    // |  true  |  false | false       |
    // |  false |  true  | false       |
    // |  false |  false | false       |
    // ⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻
    println!("-----------------------------");
    println!("{0} {0} {0}", "Banana" == "Banana");
    println!("{0} {0} {0}", "Banana" == "Orange");
    println!("{0} {0} {0}", "Apple" == "Banana");
    println!("{0} {0} {0}", "Red" == "Green");
    println!("-----------------------------");

    let bobs_drink: &str = "Coke";
    // Rust é case sensitive, vai então considerar falso
    println!(
        "Does Bob drink Coke? {}",
        bobs_drink == bobs_drink.to_lowercase()
    ); // false

    // Rust considera espaços em branco um caractere
    let bobs_drink: &str = "Coke ";
    println!("Does Bob drink Coke? {}", bobs_drink == bobs_drink); // false
    // != é usado para testar se ambos são diferentes
    // |-------------------------------|
    // |   p    |    q   | p != q      |
    // |⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻|
    // |  true  |  true  | false       |
    // |  true  |  false | true        |
    // |  false |  true  | true        |
    // |  false |  false | false       |
    // ⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻
    println!("-----------------------------");
    println!("{0} {0} {1}", !("Banana" != "Banana"), "Banana" != "Banana");
    println!("{1} {0} {1}", !("Banana" != "Orange"), "Banana" != "Orange");
    println!("{0} {1} {1}", !("Apple" != "Banana"), "Apple" != "Banana");
    println!("{0} {0} {0}", "Red" != "Red");
    println!("-----------------------------");
    // && (AND | E)
    // Os dois valores devem ser verdadeiros
    // |-------------------------------|
    // |   p    |    q   | p && q      |
    // |⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻|
    // |  true  |  true  | true        |
    // |  true  |  false | false       |
    // |  false |  true  | false       |
    // |  false |  false | false       |
    // ⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻
    // Exemplo: "Só irei ao cinema se meus amigos forem
    // e se os preços da pipoca forem baratos."
    // "Squid é marinho e toca clarinete."
    let squid_is_marine_animal: bool = true;
    let squid_loves_clarineta: bool = true;
    println!(
        "Is the Squid a marine animal and loves clarinet? {} && {} = {}",
        squid_is_marine_animal,
        squid_loves_clarineta,
        squid_is_marine_animal && squid_loves_clarineta
    );

    let squid_is_marine_animal: bool = true;
    let squid_loves_clarineta: bool = false;
    println!(
        "Is the Squid a marine animal and loves clarinet? {} && {} = {}",
        squid_is_marine_animal,
        squid_loves_clarineta,
        squid_is_marine_animal && squid_loves_clarineta
    );

    // || (ou)
    // Pelo menos uma das afirmações deve ser verdade
    // |-------------------------------|
    // |   p    |    q   | p || q      |
    // |⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻|
    // |  true  |  true  | true        |
    // |  true  |  false | true        |
    // |  false |  true  | true        |
    // |  false |  false | false       |
    // ⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻⁻
    // Exemplo: "Só saio de casa se for para ir ver meu avô
    // ou para ver a lua."
    let is_animal: bool = true;
    let is_domestic: bool = false;
    println!(
        "Is Luna the domestic cat or animal? {} || {} = {}",
        is_animal,
        is_domestic,
        is_animal || is_domestic
    );
    // -------------------------------------
    // ------------ CHARACTERS -------------
    // Unicode é um padrão de representação textual
    // para diversos sistemas ao redor do mundo.
    // Ele contem 140.000 caracteres que abrangem 150 idiomas
    // diferentes e históricos.
    // UTF (Unicode Transformation Formation), tem diversas variantes:
    // - UTF-8
    // - UTF-16
    // - UTF-32
    // Cada unidade de UTF, refere-se ao espaço que cada um ocupa, seu
    // tamanho na memória.
    // Em Rust caracteres, assim como, em muitas outras linguagens
    // usa aspas simples '', não supor mais que um único char.
    let first_initial: char = 'J';
    let rocket_emoji: char = '🚀';
    println!("---------------{rocket_emoji}--------------");
    println!("INITIAL={first_initial}");
    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        rocket_emoji.is_alphanumeric()
    );
    println!(
        "{} {}",
        first_initial.is_uppercase(),
        rocket_emoji.is_uppercase()
    );
    println!("-------------------------------");
    // -------------------------------------
    // ------------ ARRAYS -----------------
    // Também conhecido como lista, é uma coleção de valores, em um
    // tipo específico. (Homogêneo)
    // O termo técnico para os dados de um array são elementos,
    // armazenamos eles em uma ordem, no caso matemática.
    // Seu tamanho é fixo, não cresce ou diminui de tamanho.
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Em arrays mutáveis, podemos modificar o elemento, não
    // seu tamanho fixo.
    // Nesse caso uma lista de char com tamanho 6 (fixo).
    let mut emotions: [char; 6] = ['😊', '😕', '😍', '😱', '😴', '😁'];

    println!("{} {}", numbers.len(), emotions.len());
    println!("First emotion: {}", emotions[0]);
    println!("Second emotion: {}", emotions[1]);
    emotions[1] = '😌';
    println!("Second emotion: {}", emotions[1]);

    // Errors examples:
    // emotions[6];
    // |     ^^^^^^^^^^^ index out of bounds: the length
    //                  is 6 but the index is 6
    // :#? -> lista formatada
    // :? -> lista
    println!("{:#?}", emotions);
    println!("{:#?}", numbers);
    // -------------------------------------
    // -------------- dbg! -----------------
    let total: i32 = dbg!(2 * 3 + 2) * 100;

    println!("{total}");

    dbg!(emotions[0]);
    // -------------------------------------
    // -------------- Tuples ---------------
    // Tuplas suportam vários tipos em sua composição (Heterogêneo),
    // enquanto um array é homogêneo.
    let employee: (i32, &str, i32, &str, f64, &str, &str) =
        (1, "João Theodoro", 25, "Developer", 1.83, "Brazil", "SP");

    let id: i32 = employee.0;
    let name: &str = employee.1;
    let age: i32 = employee.2;
    let department: &str = employee.3;
    println!("{employee:#?}");
    println!("ID={id}, Name={name}, Age={age}, Depart={department}");

    let (id, name, age, department, height, country, state) = employee;
    dbg!(id, name, age, height, department, country, state);
    println!(
        "id={} nam={} age={} height={} depart={} country={} state={}",
        id, name, age, height, department, country, state
    );
    // -------------------------------------
    // -------------- Range ----------------
    // Range é uma sequência/intervalo de valores
    // consecutivos.
    // Usamos <inicial>..<final>, de maneira exclusiva (sem o ultimo), mas
    // temos <inicial>..=<final>, maneira inclusiva.
    // Iterator, significa percorrer um por um dos valores em uma
    // coleção.
    let month_days: std::ops::RangeInclusive<i32> = 1..=31;
    println!("{:#?}", month_days);

    for day in month_days {
        println!("day={day}");
    }

    let alphabet_lower: std::ops::RangeInclusive<char> = 'a'..='z';

    for letter in alphabet_lower {
        println!("{letter}");
    }

    let colors: [&str; 3] = ["Red", "Green", "Blue"];

    for color in colors {
        println!("{color} is a great color!");
    }
    // -------------------------------------
    // -------------- Generics -------------
    // Representa um argumento de tipo, ou seja, uma entrada
    // que na verdade é um tipo.
    // Um valor genérico pode abrigar um tipo de dado, por exemplo:
    // Box<i32>, um caixa de números inteiros de 32 bits.
    // Genérico significa no mundo real, "Não específico"
}
