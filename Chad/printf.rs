use std::io::{self, Write};

fn my_printf(format: &str, args: &[&dyn std::fmt::Display]) -> io::Result<()> {
    let mut arg_index = 0;
    let mut stdout = io::stdout();
    let mut chars = format.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('s') => {
                    if arg_index < args.len() {
                        write!(stdout, "{}", args[arg_index])?;
                        arg_index += 1;
                    }
                }
                Some('d') => {
                    if arg_index < args.len() {
                        write!(stdout, "{}", args[arg_index])?;
                        arg_index += 1;
                    }
                }
                Some('%') => {
                    write!(stdout, "%")?;
                }
                Some(other) => {
                    write!(stdout, "%{}", other)?;
                }
                None => {
                    write!(stdout, "%")?;
                }
            }
        } else {
            write!(stdout, "{}", c)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let nome = "Alice";
    let eta = 15;
    my_printf("Ciao, il mio nome è %s e ho %d anni.\n", &[&nome, &eta])?;
    my_printf("Il simbolo percentuale è %%.\n", &[])?;
    Ok(())
}
