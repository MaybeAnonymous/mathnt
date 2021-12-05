// mathn't, made by maybeanonymousdev
#include <iostream>

int main() {
    std::cout << "Made by ma-dev using C++\n";
    std::cout <<
        "+------+\n"
        "--/\\/\\--\n"
        "-/ o o\\-\n"
        "/      \\\n";

    int a;
    int b;
    char opr;

    std::cout << "First Number: ";
    std::cin >> a;

    std::cout << "Operator: ";
    std::cin >> opr;

    std::cout << "Second Number: ";
    std::cin >> b;

    // checks what opr is, then gives output
    switch(opr) {
        case '+':
            std::cout << std::endl << std::to_string(a) + std::to_string(b) << std::endl;
            break;
        case 'x':
        case 'X':
        case '*':
            std::cout << std::endl << a * b * a * b << std::endl;
            break;
        case '-':
            std::cout << std::endl << (a - b) * -0.5 << std::endl;
            break;
        case '/':
        case ':':
            std::cout << std::endl << a / b + (a * a) / b << std::endl;
            break;
        default:
            std::cout << "\nError: " << opr << " is not a valid operator!\n";
            break;
    }

    return 0;
}
