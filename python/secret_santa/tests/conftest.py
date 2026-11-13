from pathlib import Path
from ..read_csv import CsvReader
import pytest


@pytest.fixture
def data_path() -> Path:
    p = Path(__file__).parents[3].joinpath("src", "test_data")
    assert p.exists()
    return p


@pytest.fixture
def csv_reader(data_path):
    return CsvReader(data_path)


@pytest.fixture
def max_history_size() -> int:
    return 5


@pytest.fixture
def last_gift_chain() -> list[str]:
    return [
        "Kirk",
        "Todd",
        "Lisa",
        "Ned",
        "Bart",
        "Milhouse",
        "Rod",
        "Marge",
        "Luann",
        "Maggie",
        "Maude",
        "Homer",
    ]
