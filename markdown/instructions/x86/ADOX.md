> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADOX

Adds two operands and stores the result in the destination operand. The operation adds the source operand, the destination operand, and the value of the Carry Flag (CF) to the destination operand. The result is stored in the destination operand, and the Carry Flag is updated based on the result. Additionally, the overflow from the addition is stored in the OF flag.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction affects the CF and OF flags. The CF is set if there is a carry out of the most significant bit of the result. The OF is set if there is a carry into the most significant bit but not out of it, or vice versa.

To avoid incorrect results in multi-precision arithmetic, the user MUST ensure that the destination register is not used for other calculations between the `ADC` and `ADOX` sequences, as `ADOX` specifically targets the overflow flag for certain large-integer multiplication algorithms.