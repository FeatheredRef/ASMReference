> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXPANDPD

VEXPANDPD expands a packed double-precision floating-point vector by inserting zeros between the original elements. It takes a packed f64 vector and spreads its elements across a wider destination vector, filling the intervening gaps with f64 zeros.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F) to be available on the processor. 

When using an opmask register (k-mask), the behavior of the destination elements depends on the masking regime:
- Merging masking: Elements not selected by the mask SHALL retain their original values.
- Zeroing masking: Elements not selected by the mask SHALL be set to zero.

Users MUST ensure that the destination register is of a larger size than the source register to achieve the expansion effect; otherwise, elements beyond the destination's capacity are discarded. Improper alignment of memory operands MAY result in general-protection faults if alignment checks are enabled.