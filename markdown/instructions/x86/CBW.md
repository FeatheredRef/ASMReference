> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CBW

Extends the sign of the AL register to the AX register. The value of AL is sign-extended into AH.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (AL) | reg (AX) |
| imm | #I |
| memory | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It operates specifically on the low byte of the AX register; therefore, it cannot be used with any other register.

The instruction implicitly targets AL and AX. Attempting to use CBW in a context where the AX register is not accessible or is restricted by the current operating mode SHALL result in an invalid operation. Since this is a sign-extension operation, the most significant bit of AL (bit 7) determines whether AH is set to 00h or FFh.