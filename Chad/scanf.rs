use std::io;

fn my_scanf_int() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input.trim().parse::<i32>().map_err(|e| e.to_string())
}

fn main() -> Result<(), String> {
    println!("Inserisci un numero intero:");
    match my_scanf_int() {
        Ok(numero) => println!("Hai inserito: {}", numero),
        Err(errore) => println!("Errore durante la lettura: {}", errore),
    }
    Ok(())
}
