> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMULSH

Multiplies two signed packed numbers and shifts the result right by 1 bit (dividing by 2) to maintain the same precision as the inputs.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| fN, fN | fN |
| mN, fN | fN |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It requires the AVX-512 foundation extensions. The instruction operates on zmm or ymm registers; it SHALL NOT be used with xmm registers.

To avoid precision loss or unexpected results, ensure that the input operands are treated as signed integers. Incorrectly casting unsigned data to this instruction will result in incorrect arithmetic shifts. If the result of the multiplication exceeds the representable range of the destination register before the shift, an #O may occur depending on the specific AVX-512 implementation and rounding mode.