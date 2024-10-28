import pytest
from main_python import multiply_values


def test_multiply_values():
    # Test with a known input
    values = [10.0, 20.0, 30.0]
    factor = 2.0
    expected = [20.0, 40.0, 60.0]
    assert multiply_values(values, factor) == expected


def test_multiply_by_zero():
    # Test multiplying by zero
    values = [10.0, 20.0, 30.0]
    factor = 0.0
    expected = [0.0, 0.0, 0.0]
    assert multiply_values(values, factor) == expected


def test_empty_list():
    # Test with an empty list
    values = []
    factor = 2.0
    expected = []
    assert multiply_values(values, factor) == expected
