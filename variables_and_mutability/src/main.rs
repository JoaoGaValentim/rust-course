// Diretrizes
// - Uma diretriz de compilador é uma anotação que adicionamos ao
// código e que informa ao compilador como analisar o mesmo.
// - São metadados que adicionamos e personaliza a forma como o
// compilador opera e pensa.
// - Existem milhões de diretrizes existentes.
// - Podemos aplicar essas diretivas a linhas individuais, arquivos .rs
// inteiros ou funções.
// - Aplicamos a diretriz a cima da entidade escolhida.
// - Se eu aplicar uma diretriz de compilador acima,
// ela será aplicada na próxima linha.

// suprimir ou permitir passar a variável sem warning.
// #! Todo o arquivo (GLOBAL)
// #![allow(unused_variables)]

// Constantes
// - É um nome associado a um valor que não
// pode ser modificado, ou seja, nunca mudará.
// - Você é obrigado a declarar um tipo para a const.
// - Usamos `const` para definir uma constante.
// - Não podemos usar mut na const.
// - Não tem limitação de escopo, como variáveis,
// ou seja, podem ser declaradas em qualquer
// local do arquivo .rs .
// - Podem ser reaproveitadas em várias funções.
// - Definida em runtime (Quando programa é executado)
// - Não podemos armazenar o input do usuário.
const PI: f64 = 3.1415;
const PI_SQUARE: f64 = PI * PI;

// Type
// Usamos type para apelidar um tipo, mas, de acordo com
// uma necessidade de mais contexto, por exemplo:
// - Tratamento de dinheiro em programa financeiro.
// - Speed em contexto de corridas ou foguetes.
type Currency = f32;
type Speed = f64;
type Coordinates = (f64, f64);

fn main() {
    // Variáveis são espaços na memória,
    // elas guardam valores atribuídos a elas,
    // usando o sinal de = (igual, mas aqui, é atribuição).
    // Elas são identificadas por um nome próprio (identificador único),
    // sendo case-sensitive (diferencia lower (minusculas) de UPPER (maiúsculas)).
    // Se ela não existir o rust vai dar erro de compilação, o mesmo, se der
    // nomes errados.

    // let é usado para definir a variável, seguido de um espaço,
    // com nome em seguida.
    // se você coloca um valor numérico, inferido, o rust vai adivinhar,
    // ou seja, verificar se é i32 (int) ou "" (String)
    // let total_apples = 50;
    // let total_oranges = 33 + 10;
    // let total_fruits = total_apples + total_oranges;
    // variáveis com underscore antes, servem para, fazer o compilador ignorar o warning
    // e remover isso no momento de compilação.
    // let _x_oranges = 102;

    // podemos exibi-las na tela usando println!
    // dentro dos parenteses nas "", colocamos um par de {},
    // chamado de interpolação.
    // Obs.: no Rust 1.56, foi introduzida, a possibilidade de adicionar
    // elas dentro dos {} -> {my_variable}
    // println!("Olá {name}! Como vai?"); // ou
    // println!("Olá {}! Como vai?", name); // ou
    // println("Olá {0}! Você mora mesmo em {1}?", name, city);
    // Nessa temos a introdução de parâmetros posicionais,
    // que em rust, vão de 0 a N (argumentos). (toda linguagem conta de 0 a N).
    // println!("{1} {0}", total_apples, total_oranges);
    // println!("My garden has {total_apples} apples and {total_oranges} oranges.");
    // println!("This year my garden has {total_fruits} fruits.");

    // Em Rust as variáveis são por padrão imutáveis, ou seja, não permitem
    // modificar o valor em tempo de execução.
    // Para podermos torana-la mutável, e modificar seu valor em runtime,
    // usamos após o let a palavra mut, seguida do nome, tipo e valor,
    // atribuídos a variável mutável.
    // variável == valor variado com o tempo
    // rustc --explain E0384 (usado para entender o mut (E0384))
    let mut temperature_c_now = 25;
    println!("The temperature now has {temperature_c_now} ˚C.");

    // o valor pode mudar, mas ela permanece com o tipo i32,
    // ocasionando em erro de compilação se tentar usar, String, bool ou
    // float f32.
    // Em Rust o dev, precisa ser rigoroso, dado que é
    // uma linguagem bem defensiva.
    // Tudo deve ser explicitado por ele, para alcançar o resultado
    // esperado.
    temperature_c_now = 26;
    println!("The temperature now has {temperature_c_now} ˚C.");

    // Variável shadowing
    // A variável sombreada é uma redeclaração da
    // primeira, do mesmo nome.
    // Usamos ou melhor, Reusamos let, para redeclara-la,
    // invalidando (ofuscando) a primeira declaração.
    // Podemos usar esse conceito para refinar os valores de entrada,
    // ou seja, a cada sombreamento, redeclaramos a mesma variável,
    // atualizando seus dados, para assim obtermos oque queremos.
    let x = 10;
    println!("My coordinate(s) in X is {x}.");

    let x = 12.234;
    println!("My coordinate(s) in X is {x}.");

    let x = x / 12.333;
    println!("My coordinate(s) in X is {x}.");

    let x = x * 1.478731114;
    println!("My coordinate(s) in X is {x}.");

    let x = x + (x * 2.2) * 3.33;
    println!("My coordinate(s) in X is {x}.");

    // Escopo
    // É uma região ou limite aonde um nome é válido, ou seja,
    // é capaz de ser usado(a).
    // Se conecta a ideia de blocos (par de chaves {}),
    // aonde abrimos e fechamos, limitando o código.
    // Blocos representam escopos, um exemplo é a fn main,
    // aonde oque está dentro das {}, é executado só dentro dela,
    // isso é oque chamamos de escopo.
    // Ao final do escopo todas as variáveis são apagadas, ou não
    // são válidas mais, fazendo o Rust deixar isso claro.
    // tecnicamente podemos dizer: "A variável está fora do escopo."
    // posso acessar ela no interno, desde que, uma do mesmo
    // nome não esteja declarada dentro das {}.
    let user_name = "JoaoGDev"; // escopo externo
    {
        // essa não é uma shadowing, pois, está 2
        // no escopo das {}
        let user_name = "JoaoDev";
        println!("User's name is \"{user_name}\"."); // escopo interno
    }
    println!("User's name is \"{user_name}\".");

    println!("The PI const is {PI}.");
    println!("The PI² const is {PI_SQUARE}.");

    let brl: Currency = 3.15;
    let light_speed_square: Speed = 300000.0 * 300000.0;

    println!("I have in my wallet BRL {brl}.");
    println!("The Light Speed² is {light_speed_square}.");

    // Diretrizes inline
    #[allow(unused_variables)]
    let y_coords: Coordinates = (10.2, 12.3);
    #[allow(unused_variables)]
    let x = 10;
}
