> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPW

Compares two word-sized signed integers in the source operands based on a specified immediate condition and stores the result in the destination operand. The comparison is performed element-wise across the vector registers.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It REQUIRES the AVX-512BW instruction set extension.

The destination register MUST NOT be the same as the memory operand source to avoid undefined behavior in certain microarchitectures. Users SHOULD ensure that the immediate byte is a valid comparison predicate defined by the AVX-512 specification; using an unsupported immediate value MAY result in undefined behavior.