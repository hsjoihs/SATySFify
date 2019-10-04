all:
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
	./target/debug/SATySFify '\nabla_{\alpha}(P^\lambda_{\mu\nu}U^{\nu}_\beta) = U^{\nu}_\beta\nabla_{\alpha}P^\lambda_{\mu\nu} + P^\lambda_{\mu\nu} \nabla_{\alpha}U^{\nu}_\beta' > test/test010.saty
	satysfi test/test010.saty
	./target/debug/SATySFify '\frac{\partial H(a,b)}{\partial b} = \frac{b}{m}' > test/test011.saty
	satysfi test/test011.saty
	./target/debug/SATySFify '\frac{4}{\pi}\int _0^{\pi/2} \sin((2n-1)x) (\sin x)^{b} dx' > test/test012.saty
	satysfi test/test012.saty
	./target/debug/SATySFify '\int_b^c a(x-b)(x-c) dx = -\frac{a}{6}(c-b)^3' > test/test013.saty
	satysfi test/test013.saty
	./target/debug/SATySFify '\frac{1}{2\pi}\left(\int_{-1}^{0}(1+x)e^{ikx}dx+\int_{0}^{1}(1-x)e^{ikx}dx\right)' > test/test014.saty
	satysfi test/test014.saty
	./target/debug/SATySFify '-2ab\left(1+m^2\right) \left( \frac{ 2\sin  \theta\cos\theta}{L}-\frac{ 2abm \sin^3 \theta}{L^2} \right) + \frac{dm}{d\theta} \left(-\frac{4abm \sin^2 \theta}{L}+\cos\theta\right)-m\sin\theta -\cos \theta' > test/test015.saty
	satysfi test/test015.saty
	./target/debug/SATySFify '\frac{2}{ab}\frac{d^3}{d\theta^3} S_{true} = -2ab\left(1+m^2\right) \left( \frac{ 2\sin  \theta\cos\theta}{L}-\frac{ 2abm \sin^3 \theta}{L^2} \right) + \frac{dm}{d\theta} \left(-\frac{4abm \sin^2 \theta}{L}+\cos\theta\right)-m\sin\theta -\cos \theta' > test/test016.saty
	satysfi test/test016.saty
	./target/debug/SATySFify '\sin2' > test/test017.saty
	satysfi test/test017.saty
	./target/debug/SATySFify '107\le2(X+Y+Z)\le110' > test/test018.saty
	satysfi test/test018.saty
	./target/debug/SATySFify 'L^2\psi = \left(x\frac{\hbar}{i}\partial_y - y\frac{\hbar}{i}\partial_x\right)\left(x\frac{\hbar}{i}\partial_y - y\frac{\hbar}{i}\partial_x\right)\psi + \dots' > test/test019.saty
	satysfi test/test019.saty
