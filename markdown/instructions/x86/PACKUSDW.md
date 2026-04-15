> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PACKUSDW

Converts two unsigned word values from the source to signed word values and packs them into a single double word destination. The instruction converts u16 values to i16 values; since the range of u16 exceeds the range of i16, values larger than 32767 are saturated to 32767.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m4 | reg |
| reg | m4 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode as part of the SSE2 instruction set extension. It requires the use of XMM registers for operands when implemented via the `PACKUSDW` mnemonic in the context of SIMD operations; however, in the standard x86-64 GPR context, this specific mnemonic refers to the packing of word-sized data.

To avoid data corruption or unexpected results, ensure that the source operands are indeed unsigned 16-bit integers. Because saturation occurs at the i16 maximum (32767), any u16 value in the range [32768, 65535] WILL be clamped to 32767. Failure to account for this saturation WILL result in loss of precision for values exceeding the signed 16-bit limit.