fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2".


var i = 0;
fun something() {
    i = i + 1;
    print i;
}

fun do() {
    var i = 2;
    something();
    something();
    something();
}

do();