fun add2(c, d) {
    return add(0, add(c,d));
}

fun add(a, b) {
    return a + b;
}

fun printhello() {
    print "hello world";
}

print add(2,3);
printhello();
print add2(4, 5);


