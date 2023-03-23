var a = 1;
if a == 1 {
    print "1";
} else {
    print "2";
}

if true {
    print "3";
} else {
    print "4";
}

if false {
    print "5";
} else {
    print "6";
}

if a == 2 {
    print "7";
} else {
    print "8";
}

if a == 1 {
    var b = 3;
    if b == 3 {
        print "9";
    } else {
        print "10";
    }
}

var b = 3;
if a == 1
   if b == 3 
   print "11";
   else "print 12";