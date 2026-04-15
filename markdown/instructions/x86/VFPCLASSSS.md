> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSSS

Classifies a floating-point value by comparing it against a set of predefined categories and stores the resulting classification code into a general-purpose register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64/f32 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in x86-64 mode or compatibility mode. It requires the SSE3 extension set to be supported by the hardware.

The destination register is overwritten with a u32 classification value. Ensure the destination register is 64-bit to avoid truncation or unexpected behavior in 64-bit environments, although the result itself fits within a dword. Incorrectly interpreting the resulting integer code will lead to logic errors in floating-point exception handling or NaN detection.