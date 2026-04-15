> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PEXTRW

PEXTRW extracts bits from the source operand based on a mask. For each bit set to 1 in the mask, the corresponding bit from the source is extracted and packed into the destination in sequential order, starting from the least significant bit.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m2 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the BMI2 instruction set extension to be supported by the processor.

The mask is provided by a register. If the source is an m2 operand, the operation is performed on a word. Bits beyond the size of the operand in the mask are ignored. Failure to ensure the mask register is cleared of high-order bits when operating on smaller operand sizes MAY lead to unexpected behavior in specific compiler-generated sequences, although the architectural behavior is defined by the operand size of the instruction.