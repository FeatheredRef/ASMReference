> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULLQ

Multiplies two 64-bit polynomials modulo $x^{64} + x$. The operation treats the input operands as bit-vectors representing polynomials over the Galois Field GF(2).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the CPU to support the PML (Polynomial Multiplication) feature set.

The instruction operates on XMM registers; if the software attempts to use a general-purpose register or memory operand directly as a source or destination for this specific mnemonic, it SHALL result in an encoding error. Ensure that the XMM registers are properly initialized to avoid undefined behavior from stale data in the upper bits of the register.