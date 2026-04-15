> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2W

Permutes two 512-bit zmm registers into a destination zmm register based on a set of indices provided in a third zmm register. Each 32-bit element of the destination is selected from either the first or second source register, as determined by the most significant bit of the index in the index register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES AVX-512 support (specifically AVX-512VBMI).

To avoid undefined behavior, ensure that the index register contains valid indices within the range of 0 to 15 for each 32-bit lane. If the index exceeds the available elements of the source registers, the result for that element is undefined.