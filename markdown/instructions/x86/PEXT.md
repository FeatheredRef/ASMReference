> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PEXT

Extracts bits from the first operand according to the bitmask specified by the second operand. For each bit set to 1 in the mask, the corresponding bit from the source is extracted and packed into the destination in contiguous low-order positions.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| #I | imm |

DO NOT support LOCK

PEXT is available only if the processor supports the BMI2 instruction set. It is supported in 64-bit mode and compatibility mode. The operation is performed on the lower 32 or 64 bits of the registers depending on the operand size.

To avoid undefined behavior or incorrect results, ensure that the mask operand does not contain bits set beyond the operand size. Note that on certain processor families (specifically AMD Zen 2 and earlier), PEXT is implemented in microcode and may exhibit significantly higher latency compared to Intel implementations.