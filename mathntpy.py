#!/bin/python
# mathn't ported over to python by ma-dev


def main() -> int:
    print("Made by ma-dev")
    print(" +------+\n --/\\/\\--\n -/ o o\\\n /      \\")

    a: int = input("First Number: ")
    opr: str = input("Operator: ")
    b: int = input("Second Number: ")

    if opr == "+": # if adding
        print( (str(a) + str(b)) )
    elif opr == "x" or opr == "X" or opr == "*": # if multiplying
        c = (float(a) * float(b) * float(a) * float(b))
        print( f'{c:g}' )
    elif opr == "-": # if subtracting
        c = ( (float(a) - float(b)) * -1)
        print( f'{c:g}' )
    elif opr == ":" or opr == "/": # if dividing
        c = (float(a) / float(b) + (float(a) * float(a)) / float(b))
        print( f'{c:g}' )
    else: # if invalid operator
        print(f"\nError: {opr} is not a valid operator!")
    
    return 0

if __name__ == "__main__":
    main()