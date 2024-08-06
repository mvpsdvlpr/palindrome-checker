// Importaciones
// std -> standar library (cmd input and output)
// Regex manejo expresiones regulares

use std::io::{self, Write};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;


fn main() {
    let mut input = String::new();
    print!("Introduce una o mas cadenas de texto, separadas por comas:");
    io::stdout().flush().expect("Error al intentar limpiar el bufer");

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la libreria");

    // Input: Oso baboso
    let cadenas: Vec<&str> = input
        .trim() // Eliminamos los espacios en blanco y el salto de línea al final
        .split(',') // Dividimos las cadenas por comas
        .map(|s| s.trim()) // Eliminamos espacios adicionales alrededor de cada cadena
        .collect(); // Recolectamos las cadenas en un vector
    // output: osobaboso


    // Expresión regular para eliminar caracteres no alfanuméricos
    //let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let re = Regex::new(r"[^\p{L}\p{Nd}]").unwrap();


    println!("Resultados:");
    for cadena in cadenas{

        //let cadena_lowercase = cadena.to_lowercase();
        //let cadena_normalizada = re.replace_all(&cadena_lowercase, "").nfkd().collect::<String>();
        //let cadena_sin_diacriticos = cadena_normalizada.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
        //let cadena_final = re.replace_all(&cadena_lowercase, "").to_string();

        // Normalizar y eliminar diacríticos
        let cadena_normalizada: String = cadena.nfkd().filter(|c| c.is_alphanumeric()).collect();

        // Convertir a minúsculas
        let cadena_lowercase = cadena_normalizada.to_lowercase();

        // Eliminar caracteres no alfanuméricos
        let cadena_final = re.replace_all(&cadena_lowercase, "").to_string();

        if es_palindromo(&cadena_final){
            println!("\"{}\" es un palíndromo.", cadena);
        }else{
            println!("\"{}\" no es un palíndromo.", cadena);
        }
    }
}

fn es_palindromo(cadena:&str)->bool{
    let rev:String = cadena.chars().rev().collect();
    cadena == rev
}





































