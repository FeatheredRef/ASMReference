> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSLLD

Shifts the signed quadword value in the source operand to the left by the count specified in the second operand. Vacant bits are filled with zeros.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The shift count is determined by the low-order 5 bits of the source register/immediate, meaning the effective shift is modulo 32. If the shift count is 32 or greater, the behavior is determined by the mask applied to the count, not the literal value. Failure to account for the 5-bit mask may result in unexpected shift distances.