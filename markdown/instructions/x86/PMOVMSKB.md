> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMOVMSKB

This instruction moves the most significant bit of each byte in the source operand to a corresponding bit position in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | r32 |
| ymm | r32 |
| zmm | r32 |

DO NOT support LOCK

This instruction is NOT available in compatibility mode if the source operand is a ymm or zmm register; it requires the processor to be in 64-bit mode with AVX and AVX-512 extensions enabled.

The destination register is written as a dword. If the destination is a 64-bit register, the upper 32 bits are zeroed. Failure to account for the dword size of the result may lead to incorrect value interpretations when using 64-bit general-purpose registers.