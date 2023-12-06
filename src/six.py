# Taking a break from Rust

import math

EPSILON = 0.000001

def solve(time_limit, min_distance):
    b = -time_limit
    c = min_distance
    tmp = math.sqrt(b**2 - 4 * c)
    return ((-b - tmp) / 2, (-b + tmp) / 2)

def count(r):
    (l, r) = r
    return math.floor(r - EPSILON) - math.ceil(l + EPSILON) + 1


print(count(solve(7, 9)) * count(solve(15, 40)) * count(solve(30, 200)))

print(
    count(solve(41, 214)) *
    count(solve(96, 1789)) *
    count(solve(88, 1127)) *
    count(solve(94, 1055))
)

print(
    count(solve(71530, 940200))
)

print(
    count(solve(41968894, 214178911271055))
)