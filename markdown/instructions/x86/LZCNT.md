> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LZCNT

Counts the number of leading zero bits in a source operand. If the source operand is zero, the instruction returns the operand size (number of bits).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

The instruction is available in 64-bit mode and 32-bit mode. It REQUIRES the BMI1 feature flag to be supported by the processor; otherwise, on older processors, LZCNT may be interpreted as BSF (Bit Scan Forward), which exhibits different behavior regarding zero inputs (BSF leaves the destination register undefined if the source is zero).

When using LZCNT, the programmer MUST ensure that the target processor supports the BMI1 instruction set to avoid undefined behavior when the source operand is 0. Unlike BSF, LZCNT is defined for an input of 0, returning the full width of the register (e.g., 64 for r64).