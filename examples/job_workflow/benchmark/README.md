# Counted Job dispatch work

The Job recipe declares an input axis, work observations and measurement budgets for its [benchmark carrier](../../../macros/compiler/src/descriptor/bench/README.md).
The qualified binding supplies counted dispatch calls, a quadratic control, an independent fixed-axis judge and the caller's completion preflight.
The control binding supplies the same measured function in both callable positions and requires refusal.
Both bindings consume the same declaration through the root facade.

The judge independently requires measured work `[(2, 8), (4, 16), (8, 32)]` and hostile work `[(2, 16), (4, 64), (8, 256)]` under the declared ratio.
The report readers independently check those observations and require unavailable secondary measurements.
The preflight reuses `trials::row::completion()`, preserving the completion check's declared metadata, source revisions and generated provenance without another trial declaration.

Work counts observe calls made by the caller's loop.
They establish neither the generated dispatcher's internal complexity nor elapsed-time performance.
Execution and qualification belong to the [harness benchmark owner](../../../harness/src/bench/README.md).
