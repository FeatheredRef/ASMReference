> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTI64x4

Extracts four 64-bit doublewords from a 512-bit vector register, starting from the index specified by the immediate operand, and stores the result in a 256-bit vector register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | ymm register |
| #I | m64 / imm |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode. It is NOT available in 32-bit mode or compatibility mode.

The immediate operand MUST be in the range 0 to 15. If the immediate operand is outside this range, the behavior is undefined or may result in a general protection fault depending on the specific processor implementation. The extraction index defines the starting 64-bit element; specifically, if the index is $n$, the instruction extracts elements $n, n+1, n+2, n+3$ from the source. Since the destination is 256 bits (four 64-bit elements), the maximum valid starting index for a 512-bit source is 4. Indices greater than 4 will result in the extraction of elements beyond the source boundary, which are filled with zeros.