> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BTC

Copies the most-significant bit of the specified source to the carry flag (CF) and then clears the bit at the specified position in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | reg/mN |
| #I | #I |

Support LOCK

The instruction is available in 64-bit operand size and 32-bit operand size. When operating in 64-bit mode, if the destination is a 64-bit register or memory location, the bit index is interpreted as a u8.

The bit specified by the immediate must be within the range of the destination operand size. If the immediate value is greater than or equal to the size of the destination in bits, the behavior is undefined or may result in no operation depending on the processor implementation. The carry flag (CF) is the only EFLAGS register affected; other flags remain unchanged.