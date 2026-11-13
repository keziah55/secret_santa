import tomllib
from pathlib import Path


def _get_config_file(data_path: Path) -> Path:
    p = data_path.joinpath("config.toml")
    assert p.exists()
    return p


def _get_config(data_path: Path) -> dict:
    p = _get_config_file(data_path)
    return tomllib.loads(p.read_text())


def get_email_config(data_path: Path) -> dict:
    """Get dict for email settings."""

    config = _get_config(data_path)
    return config["email"]
