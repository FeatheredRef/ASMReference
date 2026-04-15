> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PDEP

PDEP (Parallel Deposit) deposits bits from the source operand into the destination operand based on the mask specified by the second source operand. For each bit set to 1 in the mask, the next available bit from the source is copied to that position in the destination; bits in the destination corresponding to 0 bits in the mask are cleared.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | reg |

DO NOT support LOCK

PDEP is only available in 64-bit mode and compatibility mode; it is NOT supported in 32-bit mode. The instruction is part of the BMI1 instruction set extension.

Implementation of PDEP on certain microarchitectures (specifically AMD Zen 2 and earlier) may be executed in software via microcode, which results in significantly higher latency compared to Intel implementations. Users SHOULD verify the target CPU architecture to avoid performance bottlenecks in timing-critical loops.