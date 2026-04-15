> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PACKSSDW

Packs signed word values into signed doubleword values. It converts signed 16-bit integers from a source to 32-bit integers in a destination, with saturation. If the signed value is greater than the maximum representable signed 32-bit integer or less than the minimum, it is replaced by the maximum or minimum signed 32-bit integer, respectively.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports SSE2. It operates exclusively on XMM registers; therefore, it is not available in standalone 64-bit mode without SSE2 support.

The instruction performs saturation. When packing from word (16-bit) to doubleword (32-bit), the values are sign-extended. Because a signed 16-bit integer always fits within a signed 32-bit integer, saturation only occurs if the operation is viewed as part of a wider packing sequence or specific SIMD lane constraints; however, for PACKSSDW specifically, the result is a sign-extension of the i16 to i32. Users MUST ensure that the destination register is correctly aligned if memory operands were supported (though this specific mnemonic utilizes xmm registers).