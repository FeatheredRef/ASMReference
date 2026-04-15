> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSLLVQ

Shifts the 64-bit elements of a packed YMM or ZMM register to the left by the count specified in a register. The shift count is masked to 63 bits.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support.

The shift count is taken from the low-order 6 bits of the specified register; however, since this is a quadword shift (64-bit), the shift count is masked to 6 bits (0-63). If the shift count is greater than or equal to 64, the result is zero. All operations are performed on the elements of the vector registers as independent 64-bit integers.