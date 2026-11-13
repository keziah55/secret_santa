#!/bin/env python3

from pathlib import Path

from secret_santa import CsvReader, SecretSanta, SantaDict, MessageSender, get_email_config

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

    csv_reader = CsvReader(args.data_path)
    santas: SantaDict = csv_reader.make_santa_dict(history_size=args.history_size)

    generator = SecretSanta(santas=santas)
    gift_chain = generator.make_gift_chain()

    sender = MessageSender(config)
    for n, santa_name in enumerate(gift_chain):
        idx = n + 1 if n < len(gift_chain)-1 else 0
        recipient = gift_chain[idx]

        santa = santas[santa_name]
        # print(f"\nSanta: {santa.name}, {santa.email_address}")
        email_address = "keziah.milligan@protonmail.com"
        sender.send_msg("Secret Santa!", santa.name, email_address, recipient)

        if n == 0:
            break

