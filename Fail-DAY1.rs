fn extrair_primeiro_ultimo_numeros(s: &str) -> Option<i32> {
    let numeros: String = s.chars().filter(|c| c.is_digit(10)).collect();
    if let Ok(numero) = numeros.parse::<i32>() {
        Some(numero)
    } else {
        None
    }
}

fn main() {
    let entrada = "treb7uchet";

    let primeiro_ultimo_numeros = entrada
        .split(|c: char| !c.is_digit(10))
        .filter(|&s| !s.is_empty())
        .filter_map(|s| extrair_primeiro_ultimo_numeros(s))
        .collect::<Vec<_>>();

    if let Some(primeiro) = primeiro_ultimo_numeros.first() {
        if let Some(ultimo) = primeiro_ultimo_numeros.last() {
        println!("Saída: {}{}", primeiro, ultimo);
    }
    }

}
