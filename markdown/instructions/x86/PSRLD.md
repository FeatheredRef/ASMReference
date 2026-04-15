> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRLD

Shifts the contents of the specified source operand to the right logically by the count specified in the ECX register. The vacancies in the most significant bits are filled with zeros.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| #I | #I |

DO NOT support LOCK

This instruction is available only in 32-bit mode and compatibility mode. It shall not be used in 64-bit mode.

The shift count is determined by the low-order 5 bits of ECX; therefore, only values from 0 to 31 are effectively used. If the source operand is a 16-bit register, the shift count is masked to 5 bits, but the shift is performed on a 16-bit value. Ensure the ECX register is properly initialized before execution to avoid unexpected shift counts.