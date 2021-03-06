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
	./target/debug/SATySFify '\left( L\psi\right]^2' > test/test020.saty
	satysfi test/test020.saty
	./target/debug/SATySFify '\left[ L\psi\right]^2' > test/test021.saty
	satysfi test/test021.saty
	./target/debug/SATySFify '\left[ L\psi\right)^2' > test/test022.saty
	satysfi test/test022.saty
	./target/debug/SATySFify 'F[f(x)] = \frac{1}{2\pi}\left(\int_{-1}^{0}(1+x)e^{ikx}dx+\int_{0}^{1}(1-x)e^{ikx}dx\right)' > test/test023.saty
	satysfi test/test023.saty
	./target/debug/SATySFify 'n(AB+BC) = 2n \sqrt{l^2\tan^2v+l^2} = \frac{2nl}{\cos v}' > test/test024.saty
	satysfi test/test024.saty
	./target/debug/SATySFify $$'n(AB+BC)-DC\' = \\frac{2nl}{\\cos v} -2nl \\tan v\\sin v' > test/test025.saty
	satysfi test/test025.saty
	./target/debug/SATySFify '\frac{d}{dt} (x, p) = \left.\left(\frac{\partial H(a,b)}{\partial b}, -\frac{\partial H(a,b)}{\partial a} \right)\right|_{a=x, b=p}' > test/test026.saty
	satysfi test/test026.saty
	./target/debug/SATySFify 'a\left.b\right.' > test/test027.saty
	satysfi test/test027.saty
	./target/debug/SATySFify 'a\left|b\right.' > test/test028.saty
	satysfi test/test028.saty
	./target/debug/SATySFify '|c|^2 = a\cdot a + 4 a\cdot b +4 b\cdot b = 16u^2 - 28u^2 + 16u^2 = 4u^2' > test/test029.saty
	satysfi test/test029.saty
	./target/debug/SATySFify $$'N_0\' = \\infty' > test/test030.saty
	satysfi test/test030.saty
	./target/debug/SATySFify 'A= \begin{matrix} 7/8 & 123/1024 & 5/1024  \\ 11/2048& 127/128 & 5/2048  \\ 1/256 & 1/16 & 239/256 \end{matrix} ' > test/test031.saty
	satysfi test/test031.saty
	./target/debug/SATySFify 'A=\left( \begin{matrix} 7/8 & 123/1024 & 5/1024  \\ 11/2048& 127/128 & 5/2048  \\ 1/256 & 1/16 & 239/256 \end{matrix} \right)' > test/test032.saty
	satysfi test/test032.saty
	./target/debug/SATySFify 'F_{\mu\nu} =\eta_{\mu\alpha}F^{\alpha\beta}\eta_{\beta\nu} =\left[\begin{matrix}   0     &  E_x/c &  E_y/c &  E_z/c \\     -E_x/c &  0     & -B_z   &  B_y    \\     -E_y/c &  B_z   &  0     & -B_x   \\     -E_z/c & -B_y   &  B_x   &  0  \end{matrix}\right]' > test/test033.saty
	satysfi test/test033.saty
	./target/debug/SATySFify 'F_{\mu\nu} =\eta_{\mu\alpha}F^{\alpha\beta}\eta_{\beta\nu} = \begin{bmatrix}   0     &  E_x/c &  E_y/c &  E_z/c \\     -E_x/c &  0     & -B_z   &  B_y    \\     -E_y/c &  B_z   &  0     & -B_x   \\     -E_z/c & -B_y   &  B_x   &  0  \end{bmatrix}' > test/test034.saty
	satysfi test/test034.saty
	./target/debug/SATySFify 'F_{\mu\nu} =\eta_{\mu\alpha}F^{\alpha\beta}\eta_{\beta\nu} = \begin{pmatrix}   0     &  E_x/c &  E_y/c &  E_z/c \\     -E_x/c &  0     & -B_z   &  B_y    \\     -E_y/c &  B_z   &  0     & -B_x   \\     -E_z/c & -B_y   &  B_x   &  0  \end{pmatrix}' > test/test035.saty
	satysfi test/test035.saty
	./target/debug/SATySFify '\begin{vmatrix}1 & 2 \\ 3 & 4 \\ \end{vmatrix}' > test/test036.saty
	satysfi test/test036.saty
	./target/debug/SATySFify '\begin{Vmatrix}1 & 2 \\ 3 & 4 \\ \end{Vmatrix}' > test/test037.saty
	satysfi test/test037.saty

# \dot and \vec are needed 
#	./target/debug/SATySFify '\dot{x}' > test/test026.saty
#	satysfi test/test026.saty

# must remove redundant {}, which is a whole lot of pain
#	./target/debug/SATySFify '1 +\int_1^{n_0} x^{-b}dx >  \sum_{n=1}^{n_0} {n^{-b}} > \int_1^{n_0+1} x^{-b}dx' > test/test026.saty
#	satysfi test/test026.saty

# \\ must be handled
