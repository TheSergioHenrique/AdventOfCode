use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn extrair_numeros(s: &str) -> Vec<i32> {
    s.chars().filter_map(|c| c.to_digit(10).map(|d| d as i32)).collect()
}

fn main() -> io::Result<()> {
    // Caminho do arquivo que você deseja ler
    let path = Path::new("advent_23_01/src/dia1.txt");

    // Tenta abrir o arquivo
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut soma_resultados = 0;
    // Itera sobre as linhas do arquivo
    for line in reader.lines() {
        // Manipula cada linha conforme necessário
        match line {
            Ok(line_content) => {
                //aqui a gente recebe o conteudo da linha. Beleza.
                let numeros = extrair_numeros(&line_content);

                if let Some(primeiro) = numeros.first() {
                    if let Some(ultimo) = numeros.last() {
                        let resultado = primeiro*10 + ultimo;
                        println!("Saída: {}{}", primeiro, ultimo);
                        soma_resultados += resultado;

                    }
                }
            }
            Err(e) => eprintln!("Erro ao ler linha: {}", e),
        }
    }
    println!("Soma total: {}", soma_resultados);
    Ok(())
}

