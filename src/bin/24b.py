# Copyright (c) 2023 Mikołaj Kuranowski
# SPDX-License-Identifier: MIT

from dataclasses import dataclass
from sys import stdin

import sympy


@dataclass
class Hailstone:
    px: int
    py: int
    pz: int
    vx: int
    vy: int
    vz: int

    @classmethod
    def parse(cls, x: str) -> "Hailstone":
        pos, _, vel = x.partition(" @ ")
        px, py, pz = pos.split(", ")
        vx, vy, vz = vel.split(", ")
        return cls(int(px), int(py), int(pz), int(vx), int(vy), int(vz))


def load_input() -> list[Hailstone]:
    return [Hailstone.parse(line) for line in stdin]


def solve(hailstones: list[Hailstone]) -> tuple[float, float, float]:
    # Symbols:
    #
    # px, py, pz - positions of a hailstone
    # vx, vy, vz - velocity of a hailstone
    #
    # rpx, rpy, rpz - positions of the rock
    # rvx, rvy, rvz - velocity of the rock
    #
    # p?, v? - position/velocity of a hailstone on some axis
    # rp?, rv? - position/velocity of the rock on some axis
    #
    # Assume that the rock and a hailstone collide at time `t`.
    # Therefore, for all axes:
    # p? + t * v? = rp? + t * rv?
    # t * v? - t * rv? = rp? - p?
    # t * (v? - rv?) = rp? - p?
    # t = (rp? - p?) / (v? - rv?)
    # t = (rpx - px) / (vx - rvx) = (rpy - py) / (vy - rvy) = (rpz - pz) / (vz - rvz)
    #
    # This must hold for all hailstones.
    # Skipping `t` (which is not needed for the solution and different for every hailstone),
    # this gives us 2 equations per hailstone. There are only 6 variables, thus examining
    # the first 3 hailstones in the input should be enough (turns out - it's not - taking first 4).
    #
    # Multiplying out the equations:
    # (rpx - px) / (vx - rvx) = (rpy - py) / (vy - rvy)
    # (rpx - px) * (vy - rvy) = (rpy - py) * (vx - rvx)
    rpx, rpy, rpz, rvx, rvy, rvz = sympy.symbols("rpx rpy rpz rvx rvy rvz")
    equations = []
    for h in hailstones[:4]:
        equations.append(sympy.Eq((rpx - h.px) * (h.vy - rvy), (rpy - h.py) * (h.vx - rvx)))
        equations.append(sympy.Eq((rpx - h.px) * (h.vz - rvz), (rpz - h.pz) * (h.vx - rvx)))
    solutions = sympy.solve(equations)
    assert solutions
    return solutions[0][rpx], solutions[0][rpy], solutions[0][rpz]


if __name__ == "__main__":
    hailstones = load_input()
    solution = solve(hailstones)
    result = sum(solution)
    print(result)
