# Edu Assembler Specification

## Registers

Every register has a size of 64-bit.

### General Purpose Register

These registers are general purpose and can be used for anything.

 * $G_0
 * $G_1
 * $G_2
 * $G_3
 * $G_4
 * $G_5
 * $G_6
 * $G_7

### Stack Pointer Register

These registers define the bounds of the stack. They both store a 64-bit pointer to the dynamic memory by default.

 * $S_B - Stores the address to the top of the stack, when used normally, it shouldn't change
 * $S_E - Stores the end of the stack, this changes

### MISC Register

 * $R - Return register, the return value of a call is stored here
 * $I - Instruction pointer, this pointer is moved over the program section, storing the address of the instruction currently beeing executed
 * $Z - Zero register, always contains the value zero, can not be overwritten

## Instructions

### TOC

#### Arithmetic - Base
 * addts $d $s $t => $d = $s + $t (assuming **signed** integers)
 * addis $s $t    => $s = $s + $t (assuming **signed** integers)
 * addtu $d $s $t => $d = $s + $t (assuming **unsigned** integers) 
 * addiu $s $t    => $s = $s + $t (assuming **unsigned** integers)

 * subts $d $s $t => $d = $s - $t (assuming **signed** integers)
 * subis $s $t    => $s = $s - $t (assuming **signed** integers)
 * subtu $d $s $t => §d = $s - $t (assuming **unsigned** integers)
 * subiu $s $t    => $s = $s - $t (assuming **unsigned** integers)


#### Arithmetic - Mult/Div - Easy

##### Multiplication
 * mults_e $d $s $t => $d = $s * $t (assuming **signed** integers) 
 * mulis_e $s $t    => $s = $s * $t (assuming **signed** integers)
 * multu_e $d $s $t => $d = $s * $t (assuming **unsigned** integers)
 * muliu_e $s $t    => $s = $s * $t (assuming **unsigned** integers)

##### Division
 * divts_e $d $r $s $t => $d = $s / $t and $r = $s % $t (assuming **signed** integers)
 * divtu_e $d $r $s $t => $d = $s / $t and $r = $s % $t (assuming **unsigned** integers)

#### Control Flow

##### Jump
 * jmp :label

##### Conditional Jump
 * jmpeq  $l $r :label => $l == $r
 * jmpne  $l $r :label => $l != $r

 * jmpgts $l $r :label => $l >  $r
 * jmpges $l $r :label => $l >= $r
 * jmplts $l $r :label => $l <  $r
 * jmples $l $r :label => $l <= $r

 * jmpgtu $l $r :label => $l >  $r
 * jmpgeu $l $r :label => $l >= $r
 * jmpltu $l $r :label => $l <  $r
 * jmpleu $l $r :label => $l <= $r

##### Function
 * cal :label => call the function at :label
 * ret $s => return

#### Memory
 * mov $t $s => Moves the contents of register $s to $t

##### Stack
 * push $d => push value in register $d onto stack
 * pop  $d => pop value from stack and put it into register $d

#### Misc
 * halt     => halt execution
 * exit  $s => exit execution, returning content of register $s 
 * print $s => print as ascii the contents of $s
 * read  $s => read  as ascii into $s
 * dump     => dumps the whole application state into the stdout
 * nop      => do nothing

#### Comment

`# `

#### Literal

A literal has to be prefixed with: `=`
