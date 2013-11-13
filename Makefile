# GNU Make 3.81
#
SHELL = /bin/sh
RUSTC = rustc

objects = docopt.so test_docopt run_tests agnostic_testee

# TODO: in fututre need to remove it and use whole agnostic test suite
AGNOSTIC_TEST_IDS = `seq 1 3`

docopt: $(objects)

docopt.so: docopt.rs
	$(RUSTC) docopt.rs

test_docopt: docopt.rs
	$(RUSTC) docopt.rs --test -o test_docopt

run_tests:
	./test_docopt

agnostic_testee: agnostic_testee.rs
	$(RUSTC) agnostic_testee.rs -L ./

run_agnostic_tests:
	python ./python_docopt/language_agnostic_test/language_agnostic_tester.py ./agnostic_testee $(AGNOSTIC_TEST_IDS)

clean:
	rm -f ./*.so ./test_docopt ./agnostic_testee 
