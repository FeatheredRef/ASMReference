> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMI2PD

Permutes two 512-bit vectors of double-precision floating-point numbers. For each double-precision element in the destination vector, the instruction selects one element from either the first or second source operand based on the indices specified by the immediate value.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmmreg, zmmreg / m512 | zmmreg |
| imm | zmmreg |

DO NOT support LOCK

This instruction is available ONLY when the processor is operating in 64-bit mode or compatibility mode. It REQUIRES AVX-512 support.

The immediate operand MUST be a valid index for the source vectors; indices outside the range of the available elements are treated as zero. Since this instruction operates on 512-bit registers, the destination register is overwritten, and the upper bits of the register are maintained if the operation is performed in a 256-bit context via VEX encoding, though VPERMI2PD is specifically a VEX/EVEX instruction primarily targeting ZMM registers. Failure to ensure the CPU supports the specific AVX-512 subset (AVX-512F) will result in an invalid opcode exception.