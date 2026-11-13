from pathlib import Path
from ..secret_santa.read_csv import CsvReader
import pytest


@pytest.fixture
def data_path() -> Path:
    p = Path(__file__).parents[2].joinpath("src", "test_data")
    assert p.exists()
    return p


@pytest.fixture
def csv_reader(data_path):
    return CsvReader(data_path)
