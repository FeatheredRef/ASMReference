> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESIMC

Performs an Advanced Encryption Standard (AES) inverse mix-columns operation on the destination operand using the source operand.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It requires the AES-NI feature set to be enabled in the processor.

The instruction operates on 128-bit XMM registers. Ensure that the target processor supports the AES instruction set extension to avoid an `#UD` (Undefined Opcode) exception. Since the operation is performed on XMM registers, it does not affect EFLAGS.