> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFPS

Multiplies a set of floating-point elements by a scalar value. The instruction scales floating-point values in a vector register by a scalar value provided in another register, utilizing the scaling factor to determine the number of elements processed.

The following table covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is ONLY available when the processor supports the AVX-512 FP16 extensions. It SHALL NOT be executed in compatibility mode.

The behavior of the operation is dependent on the vector length and the precision specified by the opcode. If the scalar operand is NaN, the resulting elements SHALL be NaN. Precision and rounding are governed by the MXCSR register.

Ensure that the destination register is not used as the scalar source if the architectural implementation does not support destructive operands for the specific encoding, as this may lead to undefined results in certain microarchitectures.

#P may be raised if the result of the multiplication is not exactly representable.
#O may be raised if the result exceeds the maximum representable value of the target floating-point format.
#U may be raised if the result is smaller than the smallest representable non-zero value.
#D may be raised if a denormalized operand is encountered.