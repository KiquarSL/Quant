p = 5        # auto-type => int
b: num = 4.5 # type num (float)
l = "some"   # type str (string)
d = true     # type logic

?! "Hello, world", "!"

# bind var types
a, b: num = 3.13, -4

# Short function
add(a: num, b: int) = a + b

# Long function
add_print(a: num, b: num) = {
    g = a + b
    !? "result: ", g
}

# bind args
sub(a, b: num) => a - b

# function with return
sub_print(a, b: num) num = {
    g = a - b
    !? "result: ", g
    => g
}

# math type expression
a = 4
b = 3
c = 2
D = b^2 - 4 a c

j = 4 (a b)