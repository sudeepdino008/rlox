var a = 1;
var switchv = false;
while true {
    if switchv {
        print a;
    }
    if a > 8 {
        break;
    }
    a=a+1;
    switchv = !switchv;
}
