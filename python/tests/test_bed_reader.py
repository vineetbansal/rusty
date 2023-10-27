from rusty.mypymodule import BedReader


def test_bed_reader():
    bed_reader = BedReader()
    assert bed_reader.foo() == 43
