> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRAQ

Shifts the elements of a 128-bit XMM register to the right by the count specified in a register, filling the vacated bits with zeros.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| reg | #I |
| m128 | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It cannot be used in compatibility mode.

The shift count is specified in the low-order 5 bits of the source register; if the register contains a value greater than or equal to 32, the shift count is masked to avoid undefined behavior. The operation is performed on each 64-bit quadword element of the XMM register independently. Failure to ensure the shift count register is correctly initialized may lead to unexpected results.