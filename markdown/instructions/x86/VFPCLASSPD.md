> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSPD

Classifies the double-precision floating-point value in the source operand according to the specified classification. The result is stored in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64 | reg/m32 |

DO NOT support LOCK

This instruction is only available in compatibility mode when targeting x86-64. It is not supported in 64-bit mode.

The destination must be a 32-bit register or memory location (dword) to accommodate the classification result. Failure to provide a 32-bit destination will result in an invalid encoding or unintended memory corruption.