> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FXRSTOR

Restores the state of the x87 FPU registers from a specified memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m108 | x87 FPU State |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It REQUIRES the memory operand to be 108 bytes aligned to avoid performance degradation, although it is not architecturally mandated.

To avoid unexpected behavior or general protection faults, ensure that the memory region `m108` is valid and accessible. The instruction restores the FPU control word, the FPU status word, the tag word, and the eight x87 FPU registers (st(0) through st(7)). Failure to provide a valid 108-byte block will result in the corruption of the FPU state.