import functools

with open("input.txt", "r") as f:
    data = f.read()
binary = ""
for char in data:
    _bin = bin(int(char, 16))[2:]
    binary += "0" * (4 - len(_bin)) + _bin
result = 0

def go(packet):
    version = int(packet[:3], 2)
    typeID = int(packet[3:6], 2)
    if typeID == 4:
        totalPacketLength, representation = 6, ""
        while True:
            representation += packet[totalPacketLength + 1: totalPacketLength + 5]
            totalPacketLength += 5
            if packet[totalPacketLength - 5] == '0':
                break
        return version, totalPacketLength, int(representation, 2)
    else:
        totalVersion, totalPacketsLength, subPacketsValue = 0, 7, []
        lengthTypeID = int(packet[6])
        if lengthTypeID == 1:
            packetNum, startingIndex = int(packet[7:7 + 11], 2), 18
            totalPacketsLength += 11
            
            while packetNum != 0:
                subPacketVersion, subPacketLength, value = go(packet[startingIndex:])
                subPacketsValue.append(value)
                totalVersion += subPacketVersion
                totalPacketsLength += subPacketLength
                startingIndex += subPacketLength
                packetNum -= 1
            
        else:
            totalBits, startingIndex = int(packet[7:7 + 15], 2), 22
            totalPacketsLength += 15
            while totalBits != 0:
                subPacketVersion, subPacketLength, value = go(packet[startingIndex:])
                subPacketsValue.append(value)
                totalVersion += subPacketVersion
                totalPacketsLength += subPacketLength
                startingIndex += subPacketLength
                totalBits -= subPacketLength
        if typeID == 0:
            result = sum(subPacketsValue)
        elif typeID == 1:
            result = functools.reduce(lambda a, b: a * b, subPacketsValue)
        elif typeID == 2:
            result = min(subPacketsValue)
        elif typeID == 3:
            result = max(subPacketsValue)
        elif typeID == 5:
            result = 1 if subPacketsValue[0] > subPacketsValue[1] else 0
        elif typeID == 6:
            result = 1 if subPacketsValue[0] < subPacketsValue[1] else 0
        else:
            result = 1 if subPacketsValue[0] == subPacketsValue[1] else 0
        return totalVersion + version, totalPacketsLength, result

_, _, result = go(binary)
print(result)