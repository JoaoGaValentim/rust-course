// O Rust usa rustc <file>.rs para compilar o código fonte
// para código de máquina (linguagem entendida pelo computador).
// O binário gerado, vai se destinar ao sistema operacional, aonde
// compilado.
// windows: .exe
// mac/Linux: nome do arquivo sem extensão
// no mac você usa file <binary> para ver dados do binário,
// como, arquitetura
fn main() {
    // Uma função é usada para atender a uma sequencia de passos
    // ela tem uma única finalidade.
    // Em rust main é o ponto de entrada, por tanto, ela é
    // declarada de maneira obrigatória
    // Devemos pensar em uma função como uma caixa de ferramentas,
    // geralmente usadas, para resolver alguma coisa.
    // fn  my_func() { ... }
    // fn é abreviação para function
    // rust usa o padrão snake_case
    // não se separa nome de funções com espaço, mas,
    // com underscore (_)
    // usamos os parenteses para introduzir parâmetros (valores de entrada da fn)
    // {} o par de chaves é um **bloco** (corpo da fn)
    // aonde conterá nosso código,
    // esse, pertence somente a esse escopo da função

    // println é usado para imprimir uma cadeia de caracteres (string) entre aspas-duplas
    // print (imprimir) + ln (linha) -> println (imprimir linha)
    // por padrão isso coloca \n (quebra de linha)
    // diferente de outras linguagens temos ! (exclamação no final),
    // usado para definir macros (uma função mais complexa)
    println!("Hello, World! I am a Rust Dev :)");
    println!("Rust is fantastic!");
    // println!("..."); // comentário de uma linha

    /*
     * Rust multiline comment
     * Posso documentar um código aqui.
     */
}
