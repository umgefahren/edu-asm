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
 * $E - Error register

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

#### Arithmetic - Shift
 * lshlt $d $s $t => $d = $s <<< $t (logical left shift)
 * lshli $s $t    => $s = $s <<< $t (logical left shift)
 * lshrt $d $s $t => $t = $s >>> $t (logical right shift)
 * lshri $s $t    => $s = $s >>> $t (logical right shift)
 * ashrt $d $s $t => $d = $s >>  $t (arithmetic right shift)
 * ashri $s $t    => $s = $s >>  $t (arithmetic right shift)

#### Arithmetic - Bitwise Logic
 * andt $d $s $t  => $d = $s & $t
 * andi $s $t     => $s = $s & $t
 * ort  $d $s $t  => $d = $s | $t
 * ori  $s $t     => $s = $s | $t
 * xort $d $s $t  => $d = $s ^ $t
 * xori $s $t     => $s = $s ^ $t
 * nott $d $s     => $d = ! $s
 * noti $s        => $s = ! $s


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
 * mov    $t $s    => Moves the contents of register $s to $t
 * load   $t $s    => Moves the contents of the memory location $s **points to** into register $t                  (word)
 * loado  $t $s $o => Moves the contents of the memory location $s **points to** into register $t, offseted by $o  (word)
 * loadb  $t $s    => Moves the contents of the memory location $s **points to** into register $t                  (byte)
 * loadbo $t $s $o => Moves the contents of the memory location $s **points to** into register $t, offseted by $o  (byte)
 * stor   $s $t    => Moves the contents of register $s into the memory location $t **points to**                  (word)
 * storo  $s $t $o => Moves the contents of register $s into the memory location $t **points to**, offseted by $o  (word)
 * storb  $s $t    => Moves the contents of register $s into the memory location $t **points to**                  (byte)
 * storbo $s $t $o => Moves the contents of register $s into the memory location $t **points to**, offseted by $o  (byte)

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

#### Syscalls

There are syscalls. Expecially to allocate and deallocate new memory pages.

#### Comment

`# `
