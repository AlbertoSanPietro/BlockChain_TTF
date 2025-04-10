// Struttura per rappresentare una coppia chiave-valore
#[derive(Debug)]
struct Entry<K, V> {
    chiave: K,
    valore: V,
}

// Struttura per rappresentare la nostra "HashMap" manuale
#[derive(Debug)]
struct ManualHashMap<K, V> {
    capacita: usize,
    tabelle: Vec<Vec<Entry<K, V>>>,
}

impl<K, V> ManualHashMap<K, V>
where
    K: Eq + std::hash::Hash + Copy,
    V: Copy,
{
    // Funzione per creare una nuova "HashMap" manuale con una data capacità
    fn nuova(capacita_iniziale: usize) -> Self {
        // Corrispondente in C (circa):
        // ManualHashMap* mappa = (ManualHashMap*)malloc(sizeof(ManualHashMap));
        // mappa->capacita = capacita_iniziale;
        // mappa->tabelle = (Vec<Entry<K, V>>**)malloc(sizeof(Vec<Entry<K, V>>*) * capacita_iniziale);
        // for (int i = 0; i < capacita_iniziale; i++) {
        //     mappa->tabelle[i] = NULL; // Inizialmente ogni "bucket" è vuoto
        // }
        ManualHashMap {
            capacita: capacita_iniziale,
            tabelle: vec![Vec::new(); capacita_iniziale],
        }
    }

    // Funzione per calcolare l'indice (hash) di una chiave
    fn calcola_indice(&self, chiave: &K) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        chiave.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacita
    }

    // Funzione per inserire una nuova coppia chiave-valore nella "HashMap"
    fn inserisci(&mut self, chiave: K, valore: V) {
        let indice = self.calcola_indice(&chiave);
        let bucket = &mut self.tabelle[indice];

        // In C, se non ci fosse una lista in `tabelle[indice]`, si creerebbe un nuovo nodo:
        // struct Entry* nuovo_nodo = (struct Entry*)malloc(sizeof(struct Entry));
        // nuovo_nodo->chiave = chiave;
        // nuovo_nodo->valore = valore;
        // nuovo_nodo->prossimo = NULL;
        // mappa->tabelle[indice] = nuovo_nodo;
        //
        // Se ci fosse già una lista, si aggiungerebbe il nuovo nodo all'inizio (o alla fine):
        // nuovo_nodo->prossimo = mappa->tabelle[indice];
        // mappa->tabelle[indice] = nuovo_nodo;

        // In Rust, aggiungiamo semplicemente un nuovo `Entry` al `Vec` nel bucket
        bucket.push(Entry { chiave, valore });
    }

    // Funzione per ottenere il valore associato a una chiave
    fn ottieni(&self, chiave: &K) -> Option<&V> {
        let indice = self.calcola_indice(chiave);
        let bucket = &self.tabelle[indice];

        // In C, si scorrerebbe la lista concatenata in `mappa->tabelle[indice]`
        // struct Entry* corrente = mappa->tabelle[indice];
        // while (corrente != NULL) {
        //     if (corrente->chiave == chiave) {
        //         return &corrente->valore;
        //     }
        //     corrente = corrente->prossimo;
        // }
        // return NULL;

        // In Rust, iteriamo sul `Vec` nel bucket e cerchiamo la chiave
        for entry in bucket.iter() {
            if entry.chiave == *chiave {
                return Some(&entry.valore);
            }
        }
        None
    }
}

fn main() {
    // Creiamo una nuova "HashMap" manuale con capacità 10
    let mut la_mia_mappa: ManualHashMap<String, i32> = ManualHashMap::nuova(10);
    println!("Mappa creata: {:?}", la_mia_mappa);

    // Inseriamo alcune coppie chiave-valore
    la_mia_mappa.inserisci("uno".to_string(), 1);
    la_mia_mappa.inserisci("due".to_string(), 2);
    la_mia_mappa.inserisci("tre".to_string(), 3);
    println!("Mappa dopo inserimenti: {:?}", la_mia_mappa);

    // Otteniamo dei valori usando le chiavi
    if let Some(valore) = la_mia_mappa.ottieni(&"due".to_string()) {
        println!("Il valore per 'due' è: {}", valore);
    } else {
        println!("La chiave 'due' non è stata trovata.");
    }

    if let Some(valore) = la_mia_mappa.ottieni(&"quattro".to_string()) {
        println!("Il valore per 'quattro' è: {}", valore);
    } else {
        println!("La chiave 'quattro' non è stata trovata.");
    }
}
