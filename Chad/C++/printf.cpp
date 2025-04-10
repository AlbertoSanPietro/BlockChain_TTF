#include <iostream>
#include <vector>
#include <string>
#include <stdarg.h>

void my_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);

    for (const char* p = format; *p; ++p) {
        if (*p == '%') {
            ++p;
            switch (*p) {
                case 's': {
                    const char* s = va_arg(args, const char*);
                    std::cout << s;
                    break;
                }
                case 'd': {
                    int d = va_arg(args, int);
                    std::cout << d;
                    break;
                }
                case '%': {
                    std::cout << '%';
                    break;
                }
                default: {
                    std::cout << '%' << *p;
                    break;
                }
            }
        } else {
            std::cout << *p;
        }
    }

    va_end(args);
}

int main() {
    const char* nome = "Alice";
    int eta = 15;
    my_printf("Ciao, il mio nome è %s e ho %d anni.\n", nome, eta);
    my_printf("Il simbolo percentuale è %%\n");
    return 0;
}
