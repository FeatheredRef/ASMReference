> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP28SS

Computes an approximate reciprocal of a single-precision floating-point value. The instruction calculates $2^{28} / \text{source}$ and stores the result in the destination.

The table below covers what the source and destinations can be.

| Source | Destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available only when the processor is operating in 64-bit mode or compatibility mode. It requires the VEX encoding prefix; without it, the instruction is invalid.

The result is an approximation and SHALL NOT be used for high-precision requirements without subsequent refinement using the Newton-Raphson method (typically via `VRSQRTSS` or `VRCPSS` iterations). The precision of the approximation is limited to a relative error of approximately $2^{-12}$. If the input is a signaling NaN, it SHALL trigger #I; if the input is infinity or zero, the result is $0.0$.