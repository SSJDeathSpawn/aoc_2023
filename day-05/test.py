#!/usr/bin/env python3

from __future__ import annotations

import sys
import types
from pprint import pprint
from typing import Self, Iterable

import more_itertools
import pytest
import re
from beartype import beartype
from dataclasses import dataclass
from loguru import logger as log
from functools import lru_cache
from collections import Counter, defaultdict
from more_itertools import batched


def parse(pattern: str, data: str) -> list[str]:
    groups = re.fullmatch(pattern, data).groups()
    if not groups:
        log.error("Could not match pattern {} for data {}", pattern, data)
    return list(groups)

def parse_to_int(pattern: str, data: str) -> list[int]:
    return list(map(int, parse(pattern, data)))

def get_lines(filename: str) -> list[str]:
    with open(filename, "r") as fp:
        return fp.read().splitlines()

def parse_by_nl(lines: list[str]) -> list[list[str]]:
    res = []
    curr = []
    for line in lines:
        if not line.strip():
            assert curr
            res.append(curr)
            curr = []
        else:
            curr.append(line.strip())
    if curr:
        res.append(curr)
    return res

def fail(msg: str) -> None:
    print(f"ASSERTION FAILED: {msg}", file=sys.stderr)
    assert 0, msg

@beartype
class Problem:

    @staticmethod
    def intersect(src: tuple[int, int], dst: tuple[int, int]) -> dict[str, list[tuple[int, int]]]:
        src1, src2 = src
        dst1, dst2 = dst
        result = {
            "in": [],
            "left_union": [],
            "right_union": [],
        }
        assert src1 < src2 or fail(f"illegal left: {src1}")
        assert dst1 < dst2 or fail(f"illegal right: {dst1}")
        if src == dst:  # equals
            result["in"] = [dst]
        elif dst2 <= src1 or src2 <= dst1:  # precedes/i, meets/i
            result["left_union"] = [src]
            result["right_union"] = [dst]
        elif src1 < dst1 and dst2 < src2:  # dominates
            result["in"] = [dst]
        elif dst1 < src1 and src2 < dst2:  # dominated
            result["in"] = [src]
            result["right_union"] = [
                (dst1, src1),
                (src2, dst2),
            ]
        elif src1 < dst1 < src2 < dst2:  # overlaps
            result["in"] = [(dst1, src2)]
            result["left_union"] = [(src1, dst1)]
            result["right_union"] = [(src2, dst2)]
        elif dst1 < src1 < dst2 < src2:  # overlapped
            result["in"] = [(src1, dst2)]
            result["right_union"] = [(dst1, src1)]
            result["left_union"] = [(dst2, src2)]
        elif src1 == dst1 and src2 < dst2:  # starts
            result["in"] = [src]
            result["right_union"] = [(src2, dst2)]
        elif src1 == dst1 and src2 > dst2:  # started
            result["in"] = [dst]
            result["left_union"] = [(dst2, src2)]
        elif src2 == dst2 and dst1 < src1:  # finishes
            result["in"] = [src]
            result["right_union"] = [(dst1, src1)]
        elif src2 == dst2 and src1 < dst1:  # finished
            result["in"] = [dst]
            result["left_union"] = [(src1, dst1)]
        else:
            fail(f"Unknown case: {src=} {dst=}")
        return result

    def __init__(self: Self, filename: str) -> None:
        self.filename: str = filename
        self.lines: list[str] = get_lines(filename)
        self.rows: int = len(self.lines)
        self.cols: int = len(self.lines[0])

        raw_maps: list[list[str]] = parse_by_nl(self.lines)

        assert len(raw_maps[0]) == 1
        assert re.fullmatch(r"seeds: (\d+)(?: \d+)*", raw_maps[0][0])

        self.seeds = [int(i) for i in re.fullmatch(r"seeds: (.*)", raw_maps[0][0]).groups()[0].split(" ")]
        self.maps = []
        for idx in range(1, len(raw_maps)):
            tokens = raw_maps[idx]
            assert tokens[0].endswith("map:")
            inted = [[int(i) for i in t.split(" ")] for t in tokens[1:]]
            self.maps.append((tokens[0].split(" ")[0], inted))

        self.seed_intervals = [(x, x + y) for x, y in more_itertools.batched(self.seeds, 2)]

        idx = 0
        intervals = sorted(self.seed_intervals[::])
        for name, tokens in self.maps:
            new_intervals = []
            mn = 6969696969
            mx = -6969696969

            for triplet in sorted(tokens, key=lambda trip: trip[1]):
                origin, destination = Problem.parse_triplet(*triplet)
                mn = min(mn, origin[0])
                mx = max(mx, origin[1])

            whole_interval = (mn, mx)

            for itv in intervals:
                intersection = Problem.intersect(itv, whole_interval)
                new_intervals.extend(intersection["left_union"])

                for triplet in sorted(tokens, key=lambda trip: trip[1]):
                    origin, destination = Problem.parse_triplet(*triplet)
                    displacement = triplet[0] - triplet[1]
                    for xx, yy in intersection["in"]:
                        intersection2 = Problem.intersect(origin, (xx, yy))
                        for xxx, yyy in intersection2["in"]:
                            new_interval = (xxx + displacement, yyy + displacement)
                            new_intervals.append(new_interval)

            intervals = list(set(new_intervals))
            idx += 1
        pprint(sorted(intervals)[0][0])

    @staticmethod
    def parse_triplet(first: int, second: int, third: int) -> tuple[tuple[int, int], tuple[int, int]]:
        cardinality = third

        src_start_idx = second
        dst_start_idx = first

        return (src_start_idx, src_start_idx + cardinality), (dst_start_idx, dst_start_idx + cardinality)

    @staticmethod
    def test_sandbox() -> None:
        p = Problem("actual")  # 219529182
        # p = Problem("example")


def test_sandbox():
    print()
    Problem.test_sandbox()


for name, obj in list(globals().items()):
    if isinstance(obj, (types.FunctionType, types.MethodType)):
        globals()[name] = beartype(obj)

