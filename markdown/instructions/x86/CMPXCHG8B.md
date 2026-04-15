> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPXCHG8B

Compares the value of `EDX:EAX` with the value of a destination `m8`. If the values are equal, the value of `EDX:EAX` is loaded into the destination `m8` (no change to the memory). If the values are not equal, the value of `m8` is loaded into `EDX:EAX` and the ZF flag is cleared.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | m8 |
| imm | #I |
| memory | #I |

Support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 64-bit mode, the instruction operates on the `EDX:EAX` register pair and an 8-byte memory location.

The instruction MUST be used with a memory operand for the destination. Because it atomically compares and swaps a 64-bit value using 32-bit registers, the memory operand SHOULD be aligned on an 8-byte boundary to avoid potential performance penalties or atomicity issues across cache line boundaries.

Failure to initialize `EDX:EAX` before calling `CMPXCHG8B` will lead to unpredictable comparison results, as the instruction relies on the current state of these registers to determine the "expected" value.