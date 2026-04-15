> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KMOVQ

Moves a qword from a source operand to a destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |
| r64 | m8 |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It REQUIRES the support of the AVX extension.

The instruction operates on registers and memory; however, it does NOT affect the EFLAGS register. When using memory operands, the operation MUST be naturally aligned to the operand size to avoid performance penalties or faults depending on the alignment check flag.