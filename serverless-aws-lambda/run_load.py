import requests as r
import argparse
import random


def main(args):
    for i in range(1000):
        test_input = {"name": "philipp", "age": random.randint(20, 40)}
        r.post(args.url, json=test_input)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", type=str)
    args, _ = parser.parse_known_args()
    return args


if __name__ == "__main__":
    args = parse_args()
    main(args)
