> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMADD52HUQ

Multiplies two 52-bit unsigned integers from the source operands, adding the resulting 104-bit product to the destination operand. The instruction performs a multiply-accumulate operation specifically tailored for 52-bit precision unsigned integers.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support and specifically the AVX-512DQ instruction set extension.

The operation is performed on unsigned 52-bit integers; any bits provided in the source registers beyond the 52nd bit SHALL be ignored. Failure to clear higher-order bits in the input registers may lead to logically incorrect results if the software expects 64-bit multiplication. The destination register MUST be properly initialized to zero if the intended operation is a simple multiplication rather than an accumulation.