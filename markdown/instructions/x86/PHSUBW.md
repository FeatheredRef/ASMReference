> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHSUBW

Subtracts two packed signed word values from the source and destination, storing the result in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The destination register is overwritten by the result of the subtraction. Since this is a packed operation, the subtraction is performed for each pair of signed word elements independently. No flags are affected. 

To avoid unexpected results, ensure that the operands are treated as signed integers (i16). Using this instruction on unsigned data will produce mathematically correct results for two's complement arithmetic, but the conceptual interpretation of the result will be signed.