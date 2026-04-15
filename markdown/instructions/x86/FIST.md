> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FIST

Stores the truncated integer part of the ST(0) floating-point value into the specified destination. The instruction rounds the value toward zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(0) | m16, m32 |
| ST(0) | r16, r32 |

DO NOT support LOCK

FIST is only available in x86-64 when executing in compatibility mode. It is not supported in 64-bit mode.

The destination must be a 16-bit or 32-bit register or memory location; attempting to use a 64-bit destination will result in an invalid operand. If the value in ST(0) is not representable as a signed integer of the destination size, the instruction SHALL trigger an #O exception. The value in ST(0) remains unchanged after execution.