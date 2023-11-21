import pytest
import chat_py_demo


def test_sum_as_string():
    assert chat_py_demo.sum_as_string(1, 1) == "2"
