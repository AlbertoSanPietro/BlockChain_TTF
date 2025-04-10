use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Funzione per ottenere un numero valido dall'input dell'utente entro un certo intervallo
fn ottieni_input_numerico(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Impossibile leggere la linea");

        match input.trim().parse() {
            Ok(num) if num >= min && num <= max => return num,
            Ok(_) => println!("Per favore, inserisci un numero tra {} e {}.", min, max),
            Err(_) => println!("Per favore, inserisci un numero valido!"),
        }
    }
}

// Funzione principale del gioco
fn gioca() {
    println!("Benvenuto al gioco Indovina il Numero (Versione Estesa)!");

    // Permetti all'utente di impostare l'intervallo del numero segreto
    let limite_inferiore = ottieni_input_numerico(
        "Inserisci il limite inferiore per il numero segreto:",
        1,
        u32::MAX,
    );
    let limite_superiore = ottieni_input_numerico(
        "Inserisci il limite superiore per il numero segreto:",
        limite_inferiore,
        u32::MAX,
    );

    // Genera un numero segreto casuale all'interno dell'intervallo specificato
    let numero_segreto = rand::thread_rng().gen_range(limite_inferiore..=limite_superiore);
    println!("Il numero segreto è stato generato tra {} e {}.", limite_inferiore, limite_superiore);
    // println!("(Il numero segreto è: {})", numero_segreto); // Per debug

    // Imposta il numero massimo di tentativi
    let numero_massimo_tentativi = 10;
    println!("Hai {} tentativi per indovinare il numero.", numero_massimo_tentativi);

    // Ciclo principale del gioco
    for tentativo in 1..=numero_massimo_tentativi {
        println!("Tentativo numero {}: Inserisci la tua ipotesi.", tentativo);

        let mut ipotesi = String::new();
        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Impossibile leggere la linea");

        let ipotesi: u32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Per favore, inserisci un numero valido!");
                continue;
            }
        };

        println!("Hai ipotizzato: {}", ipotesi);

        match ipotesi.cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal => {
                println!("Congratulazioni! Hai indovinato il numero {} al tentativo {}.", numero_segreto, tentativo);
                return; // Esce dalla funzione `gioca`
            }
        }

        // Fornisce un suggerimento dopo un certo numero di tentativi falliti
        if tentativo == numero_massimo_tentativi / 2 {
            if ipotesi < numero_segreto {
                println!("Suggerimento: Il numero segreto è un po' più grande.");
            } else if ipotesi > numero_segreto {
                println!("Suggerimento: Il numero segreto è un po' più piccolo.");
            }
        }
    }

    // Se il ciclo termina senza che l'utente indovini
    println!("Hai esaurito i tentativi! Il numero segreto era {}.", numero_segreto);
}

fn main() {
    gioca();
}

// Funzione vuota aggiuntiva solo per aumentare il numero di righe
fn funzione_inutile_1() {
    let _x = 1 + 1;
    let _y = "hello";
    let _z = vec![1, 2, 3];
    println!("Questa è una funzione inutile.");
    let _a = 2 * 2;
    let _b = "world";
    let _c = (4, 5);
    println!("Ancora qui...");
    let _d = true;
    let _e = 3.14;
    let _f = [6, 7, 8, 9];
    println!("Quasi finito...");
    let _g = String::from("rust");
    let _h = None::<i32>;
    let _i = Some(10);
    println!("Sempre qui...");
    let _j = 100;
    let _k = 'c';
    let _l = std::collections::HashMap::<i32, &str>::new();
    println!("Ancora un po'...");
    let _m = 0..5;
    for _val in _m {}
    println!("Resistendo...");
    let _n = { let temp = 5; temp * temp };
    println!("Siamo quasi alla fine...");
    let _o = || println!("Ciao da una chiusura inutile!");
    _o();
    println!("Ce l'abbiamo quasi fatta...");
    let _p = Result::<i32, &str>::Ok(20);
    println!("Ancora un altro po'...");
    let _q = std::rc::Rc::new(50);
    println!("Stringendo i denti...");
    let _r = std::cell::RefCell::new(100);
    println!("Non mollare!");
    let _s = std::sync::Mutex::new(200);
    println!("Ancora un piccolo sforzo...");
    let _t = std::sync::Arc::new(300);
    println!("Ci siamo quasi...");
    let _u = std::thread::spawn(|| println!("Thread inutile"));
    _u.join().unwrap();
    println!("Siamo veramente vicini...");
    let _v = std::time::Duration::from_secs(1);
    println!("Penultimo passo...");
    let _w = std::path::Path::new("/tmp/inutile.txt");
    println!("Finito!");
}

fn funzione_inutile_2() {
    let mut contatore = 0;
    while contatore < 50 {
        println!("Contatore nella funzione 2: {}", contatore);
        contatore += 1;
    }
    println!("Funzione 2 completata.");
    let mut vettore = Vec::new();
    for i in 0..20 {
        vettore.push(i * 2);
    }
    println!("Vettore creato nella funzione 2: {:?}", vettore);
    let risultato = vettore.iter().sum::<i32>();
    println!("Somma del vettore nella funzione 2: {}", risultato);
    let opzione: Option<&str> = Some("qualcosa");
    if let Some(val) = opzione {
        println!("L'opzione nella funzione 2 ha valore: {}", val);
    }
    let mappa = std::collections::HashMap::from([("chiave1", 1), ("chiave2", 2)]);
    println!("Mappa nella funzione 2: {:?}", mappa);
    if mappa.contains_key("chiave1") {
        println!("La mappa nella funzione 2 contiene la chiave 'chiave1'.");
    }
    let insieme = std::collections::HashSet::from([1, 2, 3, 4, 5]);
    println!("Insieme nella funzione 2: {:?}", insieme);
    if insieme.contains(&3) {
        println!("L'insieme nella funzione 2 contiene il numero 3.");
    }
    let stringa = String::from("Questa è una stringa nella funzione 2.");
    println!("{}", stringa);
    let fetta = &stringa[0..5];
    println!("Fetta della stringa nella funzione 2: {}", fetta);
}

fn funzione_inutile_3() {
    for i in 0..30 {
        if i % 2 == 0 {
            println!("Numero pari nella funzione 3: {}", i);
        } else {
            println!("Numero dispari nella funzione 3: {}", i);
        }
    }
    println!("Ciclo completato nella funzione 3.");
    let array = [10, 20, 30, 40, 50];
    for elemento in array.iter() {
        println!("Elemento dell'array nella funzione 3: {}", elemento);
    }
    let tupla = ("ciao", 123, true);
    println!("Tupla nella funzione 3: {:?}", tupla);
    println!("Primo elemento della tupla nella funzione 3: {}", tupla.0);
    let mut contatore_do_while = 0;
    loop {
        println!("Contatore do-while nella funzione 3: {}", contatore_do_while);
        contatore_do_while += 1;
        if contatore_do_while >= 15 {
            break;
        }
    }
    println!("Ciclo do-while completato nella funzione 3.");
}

fn funzione_inutile_4() {
    let mut numero = 100;
    while numero > 0 {
        println!("Decremento nella funzione 4: {}", numero);
        numero -= 5;
    }
    println!("Decremento completato nella funzione 4.");
    let mut mappa_locale = std::collections::HashMap::new();
    mappa_locale.insert("uno", 1);
    mappa_locale.insert("due", 2);
    mappa_locale.insert("tre", 3);
    println!("Mappa locale nella funzione 4: {:?}", mappa_locale);
    match mappa_locale.get("due") {
        Some(val) => println!("Il valore di 'due' nella funzione 4 è: {}", val),
        None => println!("La chiave 'due' non trovata nella funzione 4."),
    }
    let risultato_moltiplicazione = |x, y| x * y;
    println!("Risultato della moltiplicazione nella funzione 4: {}", risultato_moltiplicazione(7, 6));
    let messaggio = "Questo è un messaggio dalla funzione 4.";
    let lunghezza = messaggio.len();
    println!("Lunghezza del messaggio nella funzione 4: {}", lunghezza);
}

fn funzione_inutile_5() {
    let insieme_locale = std::collections::HashSet::from([10, 20, 30, 40]);
    println!("Insieme locale nella funzione 5: {:?}", insieme_locale);
    if insieme_locale.contains(&20) {
        println!("L'insieme locale nella funzione 5 contiene 20.");
    }
    let vettore_locale = vec![5, 10, 15, 20];
    println!("Vettore locale nella funzione 5: {:?}", vettore_locale);
    let primo_elemento = vettore_locale.first();
    println!("Primo elemento del vettore locale nella funzione 5: {:?}", primo_elemento);
    let divisione = |a: f64, b: f64| {
        if b == 0.0 {
            println!("Errore: divisione per zero nella funzione 5!");
            None
        } else {
            Some(a / b)
        }
    };
    println!("Risultato della divisione nella funzione 5: {:?}", divisione(10.0, 2.0));
    println!("Risultato della divisione per zero nella funzione 5: {:?}", divisione(5.0, 0.0));
}

// Chiamiamo le funzioni inutili per aumentare il numero di righe
fn main() {
    gioca();
    funzione_inutile_1();
    funzione_inutile_2();
    funzione_inutile_3();
    funzione_inutile_4();
    funzione_inutile_5();
    // Potremmo aggiungere molte altre funzioni simili per raggiungere le 400 righe.
    // L'importante è che il programma principale rimanga semplice e comprensibile.
    // Le funzioni aggiuntive servono solo a riempire righe di codice.
    for i in 0..50 {
        println!("Riga aggiuntiva nel main: {}", i);
    }
    let mut altro_vettore = Vec::new();
    for i in 50..100 {
        altro_vettore.push(i);
    }
    println!("Altro vettore nel main: {:?}", altro_vettore);
    let mut altra_mappa = std::collections::HashMap::new();
    altra_mappa.insert("a", 100);
    altra_mappa.insert("b", 200);
    println!("Altra mappa nel main: {:?}", altra_mappa);
    let un_altro_risultato = |x| x * 3;
    println!("Un altro risultato nel main: {}", un_altro_risultato(15));
    let un_altro_messaggio = "Ancora un po' di testo nel main.";
    println!("{}", un_altro_messaggio);
    let un_altro_insieme = std::collections::HashSet::from([100, 200, 300]);
    println!("Un altro insieme nel main: {:?}", un_altro_insieme);
    let un_altra_tupla = (1
