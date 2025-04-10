#include <iostream>
#include <string>
#include <sstream>

int my_scanf_int() {
    std::string input;
    std::cout << "Inserisci un numero intero:" << std::endl;
    std::getline(std::cin, input);
    std::stringstream ss(input);
    int numero;
    if (ss >> numero) {
        return numero;
    } else {
        std::cerr << "Errore durante la lettura." << std::endl;
        return 0; // O un altro valore per indicare l'errore
    }
}

int main() {
    int numero_inserito = my_scanf_int();
    std::cout << "Hai inserito: " << numero_inserito << std::endl;
    return 0;
}
