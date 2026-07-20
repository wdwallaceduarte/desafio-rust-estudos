 // DESAFIO 1

use std::io;

fn main() {
    // Lê a linha de entrada do usuário (saldo e valor da compra)
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");

    // Converte a entrada em dois inteiros positivos
    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let saldo = valores[0];
    let compra = valores[1];

    if saldo >= compra {
        println!("Compra aprovada");
    } else {
        println!("Saldo insuficiente");
    }
} 

/* //  DESAFIO 2

use std::io;

fn main() {
    // Lê uma linha da entrada padrão
    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.len() == 2 {
            let nome = parts[0];
            let tipo = parts[1];

            println!("Welcome, {}! Your account type is {}.", nome, tipo);
        } else {
            println!("Invalid input.");
        }
    }
} */

