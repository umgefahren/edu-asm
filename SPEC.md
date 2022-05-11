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
 * addts $d $s $t
 * addis $s $t
 * addtu $d $s $t
 * addiu $d $s $t

 * subts $d $s $t
 * subis $s $t
 * subtu $d $s $t
 * subiu $d $s $t


#### Arithmetic - Mult/Div - Easy

##### Multiplication
 * mults_e $d $s $t
 * mulis_e $s $t
 * multu_e $d $s $t
 * muliu_e $s $t

##### Division
 * divis_e $d $r $s $t
 * diviu_e $d $r $s $t

#### Control Flow

##### Jump
 * jmp :label

##### Conditional Jump
 * jmpeq $s $t :label
 * jmpne $s $t :label
 * jmpgt $s $t :label
 * jmpge $s $t :label
 * jmplt $s $t :label
 * jmpge $s $t :label

##### Function
 * cal :label
 * ret $s

#### Memory
 * mov $d $s

##### Stack
 * push $d
 * pop  $d

#### Misc
 * halt  $s
 * exit  $s
 * print $s
 * read  $s
 * nop

#### Comment

`# `

#### Literal

A literal has to be prefixed with: `=`
