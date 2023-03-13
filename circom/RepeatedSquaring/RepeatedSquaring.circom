pragma circom 2.0.0;

template RepeatedSquaring(n){
    signal input x;
    signal output y;

    signal xs[n+1];
    xs[0] <== x;
    for (var i =0; i<n ;i++){
        xs[i+1] <== xs[i]*xs[i];
    }
    y <== xs[n];
}

component main{public [x]}=RepeatedSquaring(2);