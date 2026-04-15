# OoO

> Out of order execution; Runs instruction in parallel if possible, predict branches.

Back in the day, CPUs were very slow, their clock frequency was between 250 MHz and 1 GHz at the start. For the chips to become faster, Intel started increasing the clock frequency.

But eventually, CPUs couldn't get faster with this method, as the higher the frequency, with the same feature size, higher the voltage required and heat generated. Since even if the feature sizes were as small as possible the clock wouldn't rise much while maintaining the tolerable specs, they reached a "brick-wall".

To overcome this barrier, Intel started investing in a new way of computing data, and you might already know what it is — Parallelism. [multi-threading] and other methods started to become standard, but this is another topic.

Out Of Order execution, aka OoO, references the implementation of features that allow a synchronous routine to exploit properties of itself to run instructions in parallel, and to infer, predict, and pre-load and execute branches.

With its first implementation being possible due to the better [L-Cache] that P6 architecture provided, OoO delivered three features: Deep Branch Prediction, Dynamic Data Flow Analysis, and Speculative Execution.

These features significantly help the CPU to be better used. Before OoO, the pipeline of the chip would mostly not be 100%; an expensive instruction could stall the program.

Deep branch prediction, and speculative execution, allow the chip to analyse the flow, and based on inferences, load the pipeline with the most likely branch to be followed, only flushing on a prediction error.

As for Dynamic Data Flow Analysis, it is what allows the CPU to both amortize and mitigate the cost of an expensive instruction. If the data-model being followed by the instructions allows them to run in parallel, the CPU might do so.

In conclusion, OoO is a feature in CPUs that significantly increases their efficiency. You can leverage this by avoiding using inter-dependence on variables in code, and by decreasing the branch count.
