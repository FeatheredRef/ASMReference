> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AAM

ASCII Adjust for Multiplication converts the binary product in AL into two unpacked BCD digits. The instruction divides the value in AL by 10, stores the quotient in AH, and the remainder in AL.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |

DO NOT support LOCK

AAM is only available when the processor is operating in compatibility mode. It is not supported in 64-bit mode.

The instruction implicitly uses AL and AH; therefore, any data previously stored in AH SHALL be overwritten. This operation is strictly limited to 8-bit registers and cannot be used with r16, r32, or r64.