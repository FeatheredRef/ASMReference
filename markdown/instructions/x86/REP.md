> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# REP

The REP prefix repeats the following string instruction (MOVS, STOS, SCAS, CMPS) for the count specified in the RCX register. If RCX is 0, the instruction is not executed. After each iteration, RCX is decremented and the pointer registers (RSI, RDI) are incremented or decremented based on the Direction Flag (DF).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| mN | mN |
| reg | mN |
| #I | #I |

DO NOT support LOCK

The REP prefix must be used exclusively with string instructions. In x86-64, the behavior of the pointer registers (RSI, RDI) and the count register (RCX) is governed by the operand size override prefix; however, in 64-bit mode, these registers are typically treated as 64-bit unless overridden. The Direction Flag (DF) MUST be cleared (DF=0) for incrementing pointers or set (DF=1) for decrementing pointers.

To avoid infinite loops or memory access violations, the programmer SHALL ensure that RCX is initialized to the correct count and that the memory regions pointed to by RSI and RDI are valid and do not overlap in a manner that violates the specific string instruction's requirements. Failure to set DF explicitly via CLD or STD may lead to unpredictable pointer movement.