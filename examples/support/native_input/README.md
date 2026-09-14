# Native example input

This home reads bounded JSON configuration and explicitly declared environment pairs for native examples.
Each example owns its fields, process limits, subject and expectations.
This reader supplies no inherited environment or product configuration format.
`read.rs` supplies JSON and text fields independently of `environment.rs`, so examples without a process environment can use the same bounded reader.
