trace:
	RUST_LOG=cpu=trace cargo run --quiet --example cpu_trace -- ./roms/nestest.nes &> ./logs/cpu_trace.log

rom_test:
	make trace
	diff -u ./logs/cpu_trace.log ./logs/nestest_no_cycle.log | diff-so-fancy | less

.PHONY: trace_test