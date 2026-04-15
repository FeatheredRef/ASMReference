> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GF2P8MULB

Multiplies the source operand by the destination operand within the Galois Field $\text{GF}(2^8)$ using the irreducible polynomial $x^8 + x^4 + x^3 + x^2 + 1$.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m1 | reg |
| reg | m1 |

DO NOT support LOCK

This instruction IS ONLY available when the processor supports the ADX extension. It operates exclusively on 8-bit elements within 128-bit XMM registers (SIMD), though the mnemonic refers to the 8-bit byte operation. In 64-bit mode, it requires the use of XMM registers; it SHALL NOT be used with general-purpose registers for the actual computation of the Galois Field multiplication.

To avoid incorrect results, the programmer MUST ensure that the input data is aligned according to the requirements of the memory operand if applicable. Because this instruction performs carry-less multiplication, it DOES NOT affect any EFLAGS bits. Failure to utilize the correct XMM register alignment may result in a general protection fault or alignment check exception depending on the processor configuration.