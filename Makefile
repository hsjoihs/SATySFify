all:
	gcc main.c -o a.out
	./a.out 42 > test/test001.saty
	satysfi test/test001.saty
