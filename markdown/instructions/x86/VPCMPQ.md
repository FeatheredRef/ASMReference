> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPQ

Compares two quadword integer vectors according to a specified condition and sets the destination vector to all ones (mask) or all zeros based on the result of the comparison for each element.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |
| imm | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires a processor supporting the AVX-512 foundation.

The destination register MUST NOT be the same as the second source register if the first source is a memory operand to avoid undefined behavior in certain microarchitectures, although the architectural definition permits it. When using this instruction, the user SHOULD ensure that the ZMM registers are properly initialized to avoid performance penalties associated with upper-half register transitions.