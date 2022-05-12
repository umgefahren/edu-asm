# Bytecode of edu asm

An instruction is divided into these sections: 

| Name           | Instruction Identifier                       | Instruction flags                                                                                                                                                                                                                                                         | 1st argument                         | 2nd argument                         | xth argument                         |
|----------------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------|--------------------------------------|--------------------------------------|
| Description    | Identifies the instruction as unique bitcode | The flags determine how the instructions input should be interpreted.<br>The first bit determines the type of the first parameter.<br>If the corresponding bit is `0` it's treated as a register address.<br>If the corresponding bit is `1` it's treated as a immediate. | First argument location              | Second argument location             | xth argument location                |
| Size (in bits) | 16                                           | 8                                                                                                                                                                                                                                                                         | 8 (if register)<br>64 (if immediate) | 8 (if register)<br>64 (if immediate) | 8 (if register)<br>64 (if immediate) |


