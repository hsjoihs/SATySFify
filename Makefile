all:
	clang-format main.c -i
	cargo fmt
	cargo build
	./target/debug/SATySFify 42 > test/test001.saty
	satysfi test/test001.saty
	./target/debug/SATySFify '\int e^x x^3dx' > test/test002.saty
	satysfi test/test002.saty
	./target/debug/SATySFify '\frac{2}{3}' > test/test003.saty
	satysfi test/test003.saty
	./target/debug/SATySFify 'e^{x}\frac{1}{D+1} q \equiv \int e^x qdx' > test/test004.saty
	satysfi test/test004.saty
	./target/debug/SATySFify '\int_0^R e^{-4R(R-t)} \sqrt{2}dt' > test/test005.saty
	satysfi test/test005.saty
	./target/debug/SATySFify '\int_C e^{-z^2} dz =  \int_0^R e^{-(t+it)^2} \frac{dz}{dt} dt =  \int_0^R e^{-(t+it)^2} (1+i) dt' > test/test006.saty
	satysfi test/test006.saty
	./target/debug/SATySFify '\frac{2}{ab}\frac{d^3}{d\theta^3} S_{true}' > test/test007.saty
	satysfi test/test007.saty
	./target/debug/SATySFify '\int e^x x^3dx = e^x \frac{1}{D+1} x^3 = e^x (1-D+D^2-D^3 + ...) x^3 = e^x(x^3-3x^2+6x-6)' > test/test008.saty
	satysfi test/test008.saty
	./target/debug/SATySFify '\int e^x x^3dx = e^x(x^3-3x^2+6x-6) + C' > test/test009.saty
	satysfi test/test009.saty
