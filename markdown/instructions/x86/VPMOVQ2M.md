> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVQ2M

Moves a quadword from a General Purpose Register (GPR) to a specified element of a packed 64-bit integer vector in a YMM or ZMM register.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| r64 | ymm / zmm |

DO NOT support LOCK

This instruction is only available when the processor supports AVX-512 and is executing in 64-bit mode. It is not available in compatibility mode.

The destination register MUST be a YMM or ZMM register; using an XMM register will result in an invalid opcode. The specific element index within the destination vector is determined by the immediate operand, which MUST be within the range of the destination register's width. Failure to provide a valid immediate index will result in an encoding error.