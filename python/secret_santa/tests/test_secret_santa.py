from ..secret_santa import SecretSanta, pairwise_cycle
from ..data_types import SantaDict, Santa
import itertools
import pytest


@pytest.mark.parametrize("history_size", [0, 1, 3, 100])
def test_history_size(csv_reader, history_size, max_history_size, last_gift_chain):
    santas = csv_reader.make_santa_dict(history_size=history_size)

    expected_size = history_size if history_size < max_history_size else max_history_size

    for santa in santas.values():
        assert len(santa.history) == expected_size

    if history_size == 1:
        for santa_name, recip_name in pairwise_cycle(last_gift_chain):
            assert santas[santa_name].history == [
                recip_name
            ], f"{santa_name} history expected {recip_name}"


def test_negative_history_size(csv_reader):
    with pytest.raises(ValueError, match=r"history_size must be positive int, got -1"):
        csv_reader.make_santa_dict(-1)


def test_make_gift_chain(csv_reader, max_history_size):
    santas = csv_reader.make_santa_dict(history_size=max_history_size)

    assert sorted([s.id for s in santas.values()]) == list(range(len(santas)))

    generator = SecretSanta(santas=santas, quiet=True)
    gift_chain = generator.make_gift_chain()

    print()
    print(" -> ".join(gift_chain))

    assert set(gift_chain) == set(santas.keys())
    assert set(gift_chain) == set(santas.keys())
    assert len(gift_chain) == len(set(gift_chain))

    expected_pairs = [
        (name, gift_chain[(n + 1) % len(gift_chain)]) for n, name in enumerate(gift_chain)
    ]

    pairs = list(pairwise_cycle(gift_chain))

    assert pairs == expected_pairs

    for santa_name, recip_name in pairs:

        santa = santas[santa_name]
        recip = santas[recip_name]

        assert santa.group != recip.group
        assert recip.name not in santa.history

        assert generator._is_valid_recipient(santa_name, recip_name)

    # csv_reader.update_history(gift_chain=gift_chain, year=2025)


def test_known_invalid(csv_reader, monkeypatch):
    santas = csv_reader.make_santa_dict()

    def get_santa_by_id(id_num: int):
        return [s.name for s in santas.values() if s.id == id_num][0]

    generator = SecretSanta(santas=santas, quiet=True)
    generator._known_invalid.add("0-1")

    gift_chain = [get_santa_by_id(0), get_santa_by_id(1)]

    assert generator._check_gift_chain_valid(gift_chain) is None

    gift_chain.append(get_santa_by_id(2))
    assert generator._check_gift_chain_valid(gift_chain) is not None

    monkeypatch.setattr(generator, "_get_possible_recipients", lambda *args: [])
    assert generator._check_gift_chain_valid(gift_chain) is None
    assert "0-1-2" in generator._known_invalid


def test_invalid_chain_cycle(monkeypatch):
    """Test gift chain where last cannot give to first."""

    santas = {
        "Bart": Santa(id=0, name="Bart", group="Simpson", email_address=""),
        "Ned": Santa(id=1, name="Ned", group="Flanders", email_address=""),
        "Lisa": Santa(id=2, name="Lisa", group="Simpson", email_address=""),
    }

    generator = SecretSanta(santas=santas, quiet=True, max_attempts=5)

    vals = itertools.cycle(["Bart", "Ned", "Lisa"])

    def mock_random(*args):
        return next(vals)

    import random

    monkeypatch.setattr(random, "choice", mock_random)

    assert generator._make_gift_chain() is None
    assert "0-1-2" in generator._known_invalid

    with pytest.raises(Exception, match=r"Could not find valid gift chain in 5 attempts"):
        generator.make_gift_chain()


def test_get_possible_recipients():
    santas = {
        "Bart": Santa(id=0, name="Bart", group="Simpson", email_address=""),
        "Ned": Santa(id=1, name="Ned", group="Flanders", email_address=""),
        "Lisa": Santa(id=2, name="Lisa", group="Simpson", email_address=""),
    }

    generator = SecretSanta(santas=santas, quiet=True, max_attempts=5)

    with pytest.raises(
        Exception, match=r"Cannot get possible recipients with empty gift chain"
    ):
        generator._get_possible_recipients([])

    assert generator._get_possible_recipients(["Bart"]) == ["Ned"]
