> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMI2Q

Permutes two 512-bit vectors of quadwords. For each quadword in the destination vector, the instruction selects a quadword from either the first or second source vector based on the index provided in the immediate operand.

The table below specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |
| zmm reg | zmm reg |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

The immediate operand MUST be a valid index; providing an immediate value that exceeds the range of available quadwords in the source vectors will result in undefined behavior or an illegal instruction exception depending on the specific processor implementation. This instruction requires the AVX-512DQ instruction set extension to be enabled in the CPU.