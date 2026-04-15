> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVD2M

Convert doubleword integers to packed 16-bit signed integers and store the result in a memory destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | m16 |
| ymm | m16 |
| zmm | m16 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. The destination memory address MUST be aligned to the element size to avoid performance degradation, although the instruction supports unaligned accesses.

The operation involves saturated conversion; if the value of the source i32 is greater than 32767 or less than -32768, the result is clamped to the maximum or minimum signed 16-bit integer value, respectively. Failure to account for saturation may lead to logic errors in applications expecting wrap-around behavior.