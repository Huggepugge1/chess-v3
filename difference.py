input1 = set()
while (line := input()) != "":
    move, num = line.split(": ")
    input1.add((move, num))

input2 = set()
while (line := input()) != "":
    move, num = line.split(": ")
    input2.add((move, num))

print(input1.difference(input2))
print(input2.difference(input1))

"""
f4e5: 1
f4f5: 1
g4g5: 1
b6b7: 1
c7c8: 1
d4c2: 1
d4e2: 1
d4b3: 1
d4f3: 1
d4b5: 1
d4f5: 1
d4c6: 1
d4e6: 1
f7a2: 1
f7b3: 1
f7c4: 1
f7d5: 1
f7h5: 1
f7e6: 1
f7g6: 1
f7e8: 1
f7g8: 1
e1a1: 1
e1b1: 1
e1c1: 1
e1d1: 1
e1f1: 1
e1g1: 1
e1h1: 1
e1d2: 1
e1e2: 1
e1f2: 1
e1e3: 1
e1g3: 1
e1h4: 1
b4a3: 1
b4b3: 1
b4c3: 1
b4a4: 1
b4c4: 1
"""