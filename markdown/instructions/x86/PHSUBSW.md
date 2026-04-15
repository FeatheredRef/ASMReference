> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHSUBSW

Subtracts packed signed 16-bit integers from the destination operand and horizontally adds the resulting values into the lower 16-bit signed integer of each 64-bit lane. The operation processes the elements of the source and destination operands, performs a subtraction, and then performs a horizontal addition of the two 16-bit results within each 64-bit quadword.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 instruction set extension to be supported by the processor.

The result of the horizontal subtraction and addition is stored in the lowest 16 bits of each 64-bit element of the destination xmm register; the upper 16 bits of each 32-bit element within the 64-bit lane are cleared. The operation performs signed arithmetic; therefore, users MUST ensure that the input values are treated as i16 to avoid logic errors. Overflow in the horizontal addition is ignored, and the result is wrapped according to two's complement rules.