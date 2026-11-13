import pytest


@pytest.mark.parametrize("history_size", [0, 1, 3, 100])
def test_history_size(csv_reader, history_size):
    santas = csv_reader.make_santa_dict(history_size=history_size)


def test_make_gift_chain(csv_reader):
    santas = csv_reader.make_santa_dict(history_size=1)
    