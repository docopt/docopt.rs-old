rust-docopt
===========

This is a port of Python docopt library http://github.com/docopt/docopt

** WORK IS CURRENTLY IN PROGRESS **


Requirements
-------------

- Rust >= 0.8  // using bleeding-edge master branch of Rust
- GNU Make == 3.81  // you can try other versions as well


Building & tests
----------------
Run following commands to init and update git submodules::

   git submodule update --init

To build and run language agnostic tests::

    make

Run unit tests::

    make run_tests

Run only agnostic tests::

    make run_agnostic_tests

Clean::

    make clean

Agnositc tests all fail exept first, as by now almost nothing is
implemented yet.
    
** WORK IS CURRENTLY IN PROGRESS **
