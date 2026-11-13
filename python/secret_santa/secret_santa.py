from typing import Optional
import random
from .data_types import SantaDict


class SecretSanta:

    def __init__(self, santas: SantaDict, quiet: bool = False):
        self._santas: SantaDict = santas
        self._known_invalid: set[str] = set()
        self._join_char = "-"
        self._quiet = quiet

    def _gift_chain_str(self, gift_chain: list[str]) -> str:
        """Return string representation of gift chain."""
        return self._join_char.join([str(self._santas[s].id) for s in gift_chain])

    def _get_possible_recipients(self, gift_chain: list[str]) -> list[str]:
        """Return list of possible recipients for last person in gift chain."""

        if len(gift_chain) == 0:
            raise Exception("Cannot get possible recipients with empty gift chain")

        santa = self._santas[gift_chain[-1]]

        return [
            person.name
            for person in self._santas.values()
            if (
                (person.name != santa.name)  # cannot give to self
                and (person.group != santa.group)  # person not in same group as santa
                and (person.name not in gift_chain)  # not already allocated
                and (person.name not in santa.history)  # not in santa's recent history
            )
        ]

    def _is_valid_recipient(self, santa_name: str, recipient_name: str) -> bool:
        """Return True if santa can give to recipient."""

        santa = self._santas[santa_name]
        recipient = self._santas[recipient_name]
        return (santa.group != recipient.group) and (recipient_name not in santa.history)

    def make_gift_chain(self) -> list[str]:
        """Make list of names, defining gifting chain."""

        gift_chain = None
        while gift_chain is None:
            gift_chain = self._make_gift_chain()

        if not self._quiet:
            print(f"Found valid gift chain and {len(self._known_invalid)} invalid chains")

        return gift_chain

    def _make_gift_chain(self) -> Optional[list[str]]:
        """
        Make a gift chain, either valid or invalid.

        If the generated chain is invalid, this adds it to the set of known invalid chains
        and return None.

        Otherwise, return valid gift chain.
        """

        # pick santa at random
        start = random.choice(list(self._santas.keys()))
        gift_chain = [start]

        while len(gift_chain) < len(self._santas):
            recips = self._get_possible_recipients(gift_chain)

            gift_chain_str = self._gift_chain_str(gift_chain)

            # if no valid recipients or this gift chain is known to be invalid, return None
            if len(recips) == 0:
                self._known_invalid.add(gift_chain_str)
                return None
            elif gift_chain_str in self._known_invalid:
                return None

            # pick random recipient
            gift_chain.append(random.choice(recips))

        # check last in chain can give to first
        if not self._is_valid_recipient(gift_chain[-1], gift_chain[0]):
            self._known_invalid.add(self._gift_chain_str(gift_chain))
            return None

        return gift_chain
