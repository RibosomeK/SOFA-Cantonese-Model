import os
from textgrid import TextGrid, IntervalTier
from typing import NamedTuple
import csv
import argparse


def copy_tg(tg: TextGrid) -> TextGrid:
    copied = TextGrid(tg.name, float(tg.minTime), tg.maxTime)
    for tier in tg.tiers:
        copied.tiers.append(IntervalTier(tier.name, tier.minTime, tier.maxTime))
        for ivl in tier:
            copied.tiers[-1].add(ivl.minTime, ivl.maxTime, ivl.mark)
    return copied


class Pair(NamedTuple):
    new: str
    old: tuple[str, ...]


Scheme = dict[str, list[Pair]]


def _get_paris(old: list[str], new: list[str]) -> list[Pair]:
    start, end = 0, 0
    buf = ""
    ret = []
    for idx in range(len(old)):
        ph = new[idx]
        if ph != "":
            if buf != "":
                ret.append(Pair(buf, tuple(old[start:idx])))
            buf = ph
            start = idx
    if buf == "":
        raise ValueError(f"Invalid data!\nold: {old}, new: {new}")
    ret.append(Pair(buf, tuple(old[start:])))
    return ret


def read_scheme(file: str) -> Scheme:
    scheme: Scheme = {}
    with open(file, mode="r", encoding="utf-8") as fp:
        reader = csv.reader(fp)
        try:
            while True:
                word, *old = next(reader)
                _, *new = next(reader)
                while True:
                    end = old.pop()
                    if end != "":
                        old.append(end)
                        break
                scheme[word] = _get_paris(old, new)
        except StopIteration:
            pass
    return scheme


def iter_tg(tg: TextGrid) -> list[tuple[int, tuple[int, int]]]:
    """
    [
        (word_idx, (ph_start_idx, ph_end_idx))
        ...
    ]
    """
    indexes = []
    start = 0
    for idx, word_ivl in enumerate(tg.tiers[0].intervals):
        for jdx, ph_ivl in enumerate(tg.tiers[1].intervals[start:]):
            if ph_ivl.maxTime == word_ivl.maxTime:
                indexes.append((idx, (start, jdx + start)))
                start = start + jdx + 1
                break
    return indexes


def change_scheme(scheme: Scheme, tg: TextGrid) -> TextGrid:
    new_tg = copy_tg(tg)
    new_tg.tiers[1].intervals.clear()
    for word_idx, (
        start,
        end,
    ) in iter_tg(tg):
        word = tg.tiers[0][word_idx].mark
        count = 0
        if word not in scheme:
            print(f"WARN: Unknown words: {word}")
            print("Skipped")
            for idx in range(start, end+1):
                new_tg.tiers[1].intervals.append(tg.tiers[1].intervals[idx])
            continue
        for new, old in scheme[word]:
            new_tg.tiers[1].add(
                tg.tiers[1].intervals[start + count].minTime,
                tg.tiers[1].intervals[start + count + len(old) - 1].maxTime,
                new,
            )
            count += len(old)
    return new_tg


DEFAULT_OUT = "./out"
DEFAULT_SCHEME = "./configs/cantonese-two-seg.csv"


def main():
    # print("hello world!")
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "textgrids", help="The directory of textgrid files, non-recursive."
    )
    parser.add_argument(
        "-s",
        "--scheme",
        default=DEFAULT_SCHEME,
        help=f"The path of scheme file. default is {DEFAULT_SCHEME}",
    )
    parser.add_argument(
        "-o",
        "--out",
        default=DEFAULT_OUT,
        help=f"output directory. Default is {DEFAULT_OUT}",
    )

    args = parser.parse_args()
    scheme = read_scheme(args.scheme)
    scheme["SP"] = [Pair("SP", ("SP",))]
    scheme["AP"] = [Pair("AP", ("AP",))]
    if not os.path.exists(args.out):
        os.mkdir(args.out)
    for root, _, files in os.walk(args.textgrids):
        for file in files:
            name, ext = os.path.splitext(file)
            if ext != ".TextGrid":
                continue
            new_tg = change_scheme(scheme, TextGrid.fromFile(os.path.join(root, file)))
            new_tg.write(os.path.join(args.out, file))


if __name__ == "__main__":
    main()
