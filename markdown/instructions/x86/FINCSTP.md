> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FINCSTP

Increments the floating-point stack pointer (TOP) by one.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| None | TOP |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register state.

The floating-point stack pointer (TOP) is a 3-bit value. If the current value of TOP is 7, executing FINCSTP shall cause the TOP value to wrap around to 0. This behavior can lead to the inadvertent overwriting of stack elements if the programmer does not track the stack depth, potentially causing a stack overflow condition when subsequent `FPU` instructions attempt to access the stack.