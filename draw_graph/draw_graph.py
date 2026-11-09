#!/bin/env python3

from pathlib import Path
from collections import defaultdict
import itertools

import networkx as nx
import matplotlib.pyplot as plt


def read_csv(data_path: Path) -> dict[str, list[str]]:
    """Get dict of groups and members from `family.csv`"""

    csv_file = data_path.joinpath("family.csv")
    if not csv_file.exists():
        raise FileNotFoundError(f"No such file '{csv_file}'")

    groups = defaultdict(list)

    header, *lines = [line for line in csv_file.read_text().split("\n") if line]

    header = header.split(",")
    name_idx = header.index("name")
    group_idx = header.index("group")

    for line in lines:
        row = line.split(",")
        groups[row[group_idx]].append(row[name_idx])

    return groups


def draw_graph(groups: dict[str, list[str]]):

    # edges = _make_edges(groups)
    # print(edges)

    graph = nx.Graph()

    group_names = groups.keys()

    colours = dict(zip(group_names, ["r", "g", "b"]))

    for group, members in groups.items():

        all_valid_for_group = [
            name
            for _group_name in group_names
            for name in groups[_group_name]
            if _group_name != group
        ]

        for member in members:
            edges = [(member, name) for name in all_valid_for_group]

            graph.add_edges_from(edges, label=group, node_color=colours[group])

    nx.draw_networkx(graph)
    plt.legend()
    plt.show()



def _make_edges(groups: dict[str, list[str]]) -> list[tuple[str, str]]:
    """Make list of valid edges between all members."""

    edges = []
    group_names = groups.keys()

    for group, members in groups.items():

        all_valid_for_group = [
            name
            for _group_name in group_names
            for name in groups[_group_name]
            if _group_name != group
        ]

        for member in members:
            edges += [(member, name) for name in all_valid_for_group]

    return edges


if __name__ == "__main__":

    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("data_path", type=Path, help="Path to dir containing csvs")

    args = parser.parse_args()

    groups = read_csv(args.data_path)
    # print(groups)
    draw_graph(groups)
