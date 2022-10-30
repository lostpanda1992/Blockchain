///射击小游戏
///炮弹是否打中了飞机（根据坐标（x,y,z）判断）
pragma circom 2.0.3;

include "../../node_modules/circomlib/circuits/mimcsponge.circom";
include "../../node_modules/circomlib/circuits/gates.circom";
include "../../node_modules/circomlib/circuits/comparators.circom";

pragma circom 2.0.0;

template Multiplier2(){
    // Declaration of signals.  
    signal input a;  
    signal input b;  
    signal output c;  

    // Constraints.  
    c <== a * b;  
}

//This circuit multiplies in1, in2, and in3.
template Multiplier3 () {
   //Declaration of signals and components.
   signal input in1;
   signal input in2;
   signal input in3;
   signal output out;
   component mult1 = Multiplier2();
   component mult2 = Multiplier2();

   //Statements.
   mult1.in1 <== in1;
   mult1.in2 <== in2;
   mult2.in1 <== mult1.out;
   mult2.in2 <== in3;
   out <== mult2.out;
}

component main = Multiplier3();