all:
	clang-format main.c -i
	cargo build
	./target/debug/SATySFify 42 > test/test001.saty
	satysfi test/test001.saty
	./target/debug/SATySFify '\int e^x x^3dx' > test/test002.saty
	satysfi test/test002.saty
	./target/debug/SATySFify '\frac{2}{3}' > test/test003.saty
	satysfi test/test003.saty
