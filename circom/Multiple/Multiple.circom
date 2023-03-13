pragma circom 2.0.0;

template Multiple2(){
    signal input x;
    signal input y;
    signal output z;

    // constrains
    z <== x*y;
}

component main = Multiple2();