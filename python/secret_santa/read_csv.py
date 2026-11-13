from pathlib import Path
from .data_types import SantaDict, Santa


class CsvReader:

    SEP = ","

    def __init__(self, data_path: Path):
        self._data_path = data_path
        self._family_file = self._data_path.joinpath("family.csv")
        self._history_file = self._data_path.joinpath("history.csv")

    @classmethod
    def _row(cls, line: str) -> list[str]:
        return [item.strip() for item in line.split(cls.SEP) if item.strip()]

    def _read_file(self, file_path: Path) -> list[str]:
        """Read file to list of non-empty strings"""

        if not file_path.exists():
            raise FileNotFoundError(f"'{file_path}' not found")
        return [line.strip() for line in file_path.read_text().split("\n") if line.strip()]

    def make_santa_dict(self, history_size: int = 0) -> SantaDict:
        """Read all csv data and create dict of `Santa`."""

        dct = self._load_santa_data()
        self._set_history(dct, history_size=history_size)

        return dct

    def _load_santa_data(self) -> SantaDict:
        """Read `family.csv` and create dict of `Santa`."""

        header, *lines = self._read_file(self._family_file)
        header = self._row(header)

        name_idx = header.index("name")
        group_idx = header.index("group")
        email_idx = header.index("email")

        dct = {}

        for n, line in enumerate(lines):
            row = line.split(self.SEP)
            name = row[name_idx]
            dct[name] = Santa(
                id=n,
                name=name,
                group=row[group_idx],
                email_address=row[email_idx],
            )

        return dct

    def _set_history(self, santas: SantaDict, history_size: int = 0):
        """Read `history.csv` and update `santas` dict."""

        header, *lines = self._read_file(self._history_file)
        header = self._row(header)

        end = len(lines) - 1
        start = 0 if history_size > end else end - history_size

        for idx, name in enumerate(header):
            if not name:
                continue

            if name not in santas:
                raise ValueError(f"History contains unknown name '{name}'")

            for row_idx in range(start, end):
                if not lines[row_idx].strip():
                    continue

                row = self._row(lines[row_idx])
                if row[idx]:
                    santas[name].history.append(row[idx])

    def update_history(self, gift_chain: list[str], year: str):
        """Update `history.csv` with new data."""

        header, *_ = self._read_file(self._history_file)
        header = self._row(header)

        new_line_values = [year]

        for name in header:
            if not name:
                continue
            idx = gift_chain.index(name)
            if idx == len(gift_chain) - 1:
                idx = 0
            else:
                idx += 1
            new_line_values.append(gift_chain[idx])

        new_line = self.SEP.join(new_line_values) + "\n"
        with open(self._history_file, "a") as fileobj:
            fileobj.write(new_line)
