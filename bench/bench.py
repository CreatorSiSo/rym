import sys
sys.set_int_max_str_digits(9999999)


def fib_iter(n):
    current = 0
    next = 1

    i = 0
    while True:
        if i >= n:
            break

        new = next + current
        current = next
        next = new
        i = i + 1

    return current


N = 99999
print("fib_iter(", N, ") => ")
print(fib_iter(N))
