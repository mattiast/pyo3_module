import sample_module
import numpy as np
import pytest


def test_sq():
    j = sample_module.Juttu(5, False)
    assert j.xsq() == 25


def test_juttu():
    j = sample_module.Juttu(123, False)
    assert "123" in repr(j)


def test_add5():
    assert sample_module.add5(4) == 9
    with pytest.raises(RuntimeError):
        sample_module.add5(3)


def test_add6():
    assert sample_module.subi.kuus(4) == 10


def test_cumsum():
    x = np.ones(5)
    sample_module.cumsum_inplace(x)
    assert (x == np.arange(1, 6)).all()


def test_ev():
    x1 = sample_module.ev_presses(1.0, 50000)
    x2 = sample_module.ev_presses(2.0, 50000)
    assert x1 < x2
