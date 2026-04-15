> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXSQ

Compares two signed quadword integers and stores the maximum value in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction is available only when using the AVX-512 Foundation instructions. It requires the processor to support the AVX-512 extension set; otherwise, the instruction will trigger an invalid opcode exception.

The destination register MUST be an XMM register. Using this instruction in compatibility mode is NOT permitted if the corresponding AVX-512 feature flags are not enabled in the CR0 or CR4 control registers. To avoid undefined behavior, ensure that the operand registers are properly initialized, as this instruction operates on the lower 64 bits of the XMM registers.