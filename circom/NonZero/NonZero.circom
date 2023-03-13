template NonZero(){
    signal input in;
    signal inverse;

    1 <== inverse * in;
}

template Main(){
    signal input a; signal input b;
    component nz = NonZero();
    nz.in <== a;

    0 === a * b;
}

component main{public [a,b]}= Main()