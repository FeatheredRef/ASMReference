> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSPH

Classifies a floating-point value as a positive number, negative number, or zero.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f32/f64 | r32/r64 |

DO NOT support LOCK

This instruction is available only in compatibility mode. It operates on the x87 FPU stack or XMM registers depending on the specific implementation context of the VFP extension.

The result is returned as a classification code in the destination register; users MUST ensure the destination register is wide enough to accommodate the return value to avoid truncation. Failure to use this instruction in compatibility mode SHALL result in an invalid opcode exception.