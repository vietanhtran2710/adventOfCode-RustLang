from math import sqrt
SCANNER_NUM = 5

with open("input.txt", "r") as f:
    data = f.readlines()
beacons, total = [], 0
for i in range(SCANNER_NUM):
    scanner_beacons = []
    index = data.index("--- scanner " + str(i) + " ---\n") + 1
    while index < len(data) and data[index] != "\n":
        scanner_beacons.append(tuple(map(int, data[index].strip().split(","))))
        total += 1
        index += 1
    beacons.append(scanner_beacons)
for item in beacons:
    print(len(item))
print(total)
common = set()
for x in range(0, SCANNER_NUM - 1):
    for y in range(x + 1, SCANNER_NUM):
        pair1, pair2 = [], []
        for i in range(len(beacons[x]) - 1):
            for j in range(i + 1, len(beacons[x])):
                vector = [(beacons[x][i][k] - beacons[x][j][k]) ** 2 for k in range(3)]
                pair1.append(sqrt(sum(vector)))
        for i in range(len(beacons[y]) - 1):
            for j in range(i + 1, len(beacons[y])):
                vector = [(beacons[y][i][k] - beacons[y][j][k]) ** 2 for k in range(3)]
                pair2.append(sqrt(sum(vector)))
        i =  0
        while i < len(pair1):
            found = False
            for j in range(len(pair2)):
                if pair1[i] == pair2[j]:
                    common.add(pair1[i])
                    pair1.remove(pair1[i])
                    pair2.remove(pair2[j])
                    found = True
                    break
            if not found:
                i += 1
print(len(common))
        # if len(common) != 0:
            # print(x, y, ":", len(common))
        # common.sort()
        # print(common)
# for item in pair1.intersection(pair2):
#     print(item)
# print(len(pair1.intersection(pair2)))
# print(beacons[0])