.PHNOY: single_cycle tomasulo clean gen

single_cycle:
	@cargo run --bin single_cycle

tomasulo:
	@cargo run --bin tomasulo

gen:
	@cargo run --bin gen

clean:
	@cargo clean
	@rm data.txt
	@rm inst.txt