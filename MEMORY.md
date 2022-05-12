# Memory of edu asm

## Paging

The memory in edu asm and the edu asm vm is segmented in pages with each 65,536 bytes of memory.

One can only "allocate" memory in pages and only in OS mode of the VM.

Every page of the memory is mutable, this allows the exploration of exploits in a controlled environment and also
simulates the behaviour of modern CPUs.

## Memory model

The memory is divided into two segments, the static memory and the dynamic memory.

### Static memory

The static memory is allocated at program startup. It divides into 3 segments.

#### Code section

The code section is at the very top of the static memory. The first instruction in the pgoram section start's at `0x00`,
however this is not necessarly the instruction that is first executed.

The execution starts at the `_start:` label.

The size of the program section is clear at compile time. There will be a pseudo instruction introduced, determening the end of the code section.

#### Data section

The data section contains the global variables, statics and data, that is determind at compile time.

#### Stack

The stack start is last element of the static memory. It has a size limit, determind at runtime. There will be an execption,
when the stack exceeds this maximum size.

The stack grows from the low, to the high addresses.


### Dynamic memory

The dynamic memory is memory, allocated at runtime from the virtual operating system or, in bare metal mode, just used.
