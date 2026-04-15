> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213PH

Computes the fused negative multiply-subtract for half-precision floating-point values. It calculates the result using the formula: destination = -(source1 * source2) - source3.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires support for the AVX-512 FP16 extension.

To avoid precision loss or incorrect results, ensure that the destination register is an XMM or ZMM register compatible with the FP16 data type. Improper alignment of memory operands (if applicable in related variants) may cause general protection faults; however, this specific register-to-register form avoids memory alignment issues. The operation is performed with infinite precision for the intermediate product before the final subtraction and rounding.