> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KTESTB

Performs a bitwise logical AND between a byte-sized register and a byte-sized source operand. The result is not stored; instead, it updates the ZF (Zero Flag) and SF (Sign Flag) based on the result.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| m1 | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It is part of the AVX-512 extension set; therefore, it SHALL only be executed on processors supporting the corresponding AVX-512 feature subset.

The instruction operates on the low 8 bits of the specified registers. Ensure that the target register is properly initialized, as the operation only affects the ZF and SF flags and does not modify the destination register value. Failure to verify the AVX-512 capability of the target CPU will result in an invalid opcode exception.