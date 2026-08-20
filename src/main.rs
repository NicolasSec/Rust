
use std::{io, println};

fn exercicio1(){
    println!("Digite o ano que você nasceu: ");

    let mut ano_string: String = String::new();
    io::stdin().read_line(&mut ano_string).expect("Erro ao ler linha");

    let ano_sem_quebra_de_linha = ano_string.trim().to_string();

    let ano_u16: u16 = ano_sem_quebra_de_linha.parse().expect("erro ao converter");

    println!("Você tem {} anos de idade", 2026 - ano_u16)
}

fn exercicio2(){
    let mut valor_carro: Float = Float::new();
    valor_carro = 100;

    println!("Digite a quantidade de dias que você vai alugar");
}

fn main() {
    exercicio1();
}
