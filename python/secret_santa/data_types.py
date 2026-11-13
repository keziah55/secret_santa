from dataclasses import dataclass, field


@dataclass
class Santa:
    id: int
    name: str
    group: str
    email_address: str
    history: list[str] = field(default_factory=list)


SantaDict = dict[str, Santa]
