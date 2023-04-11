var a = 1;
while a < 3 {
    var b = 1;
    while b < 5 {
        while true {
            print b;
            if b >= 3 {
                b = b +1;
                break;
            }

            b = b + 1;
        }
    }
    a=a+1;
}
