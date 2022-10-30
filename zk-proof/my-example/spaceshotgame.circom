pragma circom 2.0.0;
// copy with circomlib/circuits/comparators.circom

template IsZero() {
    signal input in;
    signal output out;

    signal inv;

    inv <-- in!=0 ? 1/in : 0;

    out <== -in*inv +1;
    in*out === 0;
}

template IsEqual() {
    signal input in[2];
    signal output out;

    component isz = IsZero();

    in[1] - in[0] ==> isz.in;

    isz.out ==> out;
}

template Calculate() {
    signal input in[3];
    signal output out;

    signal inv1;
    signal inv2;

    in[1] * in[2] ==> inv1;
    in[0] + inv1 ==> inv2;

    inv2 ==> out;
}

template SpaceShotGame(){
    signal input xpoint; 
    signal input ypoint; 
    signal input zpoint;

    // (x,y,z,p,q,h,x0,y0,z0)
    // 前三项为炮的位置，中间三项为炮的每一项攻击距离，最后三项为炮攻击点相对于坐标轴正反向，暂不考虑完全重合情况
    signal private input attack[9];
    signal output out; // 0 or 1

    template c1 = Calculate();
    c1.in[0] <== attack[0];
    c1.in[1] <== attack[6];
    c1.in[2] <== attack[3];
    
    template c2 = Calculate();
    c2.in[0] <== attack[1];
    c2.in[1] <== attack[7];
    c2.in[2] <== attack[4];

    template c3 = Calculate();
    c3.in[0] <== attack[2];
    c3.in[1] <== attack[8];
    c3.in[2] <== attack[5];

    component eq1 = IsEqual();
    eq1.in[0] <== c1.out;
    eq1.in[1] <== xpoint;
    eq1.out === 1;

    component eq2 = IsEqual();
    eq2.in[0] <== c2.out;
    eq2.in[1] <== ypoint;
    eq2.out === 1;

    component eq 3= IsEqual();
    eq3.in[0] <== c3.out;
    eq3.in[1] <== zpoint;
    eq3.out === 1;

    out <-- (eq1.out + eq2.out + eq3.out) * 1/3;
    out === 1;
}

component main = SpaceShotGame();