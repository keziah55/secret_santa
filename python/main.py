#!/bin/env python3

from pathlib import Path

from secret_santa import generate, get_email_config


if __name__ == "__main__":

    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("data_path", type=Path, help="Path to dir of csv files defining people")
    parser.add_argument("-u", "--update", action="store_true", help="Update 'history.csv' file")
    parser.add_argument(
        "-s",
        "--history_size",
        type=int,
        help="Exclude 'history_size' recent years from potential matches",
        default=0,
    )

    args = parser.parse_args()

    config = get_email_config(args.data_path)

    generate(
        data_path=args.data_path,
        history_size=args.history_size,
        update_history=args.update,
        email_config=config,
        send_email=True,
    )
