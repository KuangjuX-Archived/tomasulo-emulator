load_file := "testcases/basic.txt"

tomasulo:
    cargo run --bin tomasulo

single:
    cargo run --bin single_cycle {{ load_file }}

clean:
    cargo clean
