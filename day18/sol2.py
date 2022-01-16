from math import floor, ceil

numbers = []

def transform_to_list(s):
    result, number, index = [], "", 0
    while index < len(s):
        if s[index] in "[]":
            if number:
                result.append(int(number))
                number = ""
            result.append(s[index])
        elif s[index] != ",":
            number += s[index]
        elif number:
            result.append(int(number))
            number = ""
        index += 1
    return result

def split(number, index):
    value = number[index]
    return number[:index] + ["[", floor(value / 2), ceil(value / 2), "]"] + number[index + 1:]

def explode(number, start, end):
    i = start - 1
    while i >= 0:
        if type(number[i]) == type(1):
            number[i] += number[start + 1]
            break
        i -= 1
    i = end + 1
    while i < len(number):
        if type(number[i]) == type(1):
            number[i] += number[end - 1]
            break
        i += 1
    number = number[:start] + [0] + number[end + 1:]
    return number

def reduce(number):
    while True:
            action, nestedPair = False, 0
            for i in range(len(number)):
                if number[i] == "[":
                    nestedPair += 1
                elif number[i] == "]":
                    nestedPair -= 1
                elif type(number[i + 1]) == type(1):
                    if nestedPair > 4:
                        number = explode(number, i - 1, i + 2)
                        action = True
                        break
            if action:
                continue
            for i in range(len(number)):
                if type(number[i]) == type(1) and number[i] >= 10:
                    number = split(number, i)
                    action = True
                    break
            if not action:
                break
    return number

def magnitude(number):
    stack = []
    for item in resultNumber:
        if item == "[" or type(item) == type(1):
            stack.append(item)
        else:
            value1 = stack.pop()
            value2 = stack.pop()
            stack.pop()
            stack.append(value2 * 3 + value1 * 2)
    return stack[0]

with open("input.txt", "r") as f:
    data = f.readlines()
for line in data:
    numbers.append(transform_to_list(line.strip()))
resultNumber = numbers[0]
_max = 0
for i in range(1, len(numbers) - 1):
    for j in range(i + 1, len(numbers)):
        resultNumber = ["["] + numbers[i] + numbers[j] + ["]"]
        resultNumber = reduce(resultNumber)
        mag = magnitude(resultNumber)
        if mag > _max:
            _max = mag
        resultNumber = ["["] + numbers[j] + numbers[i] + ["]"]
        resultNumber = reduce(resultNumber)
        mag = magnitude(resultNumber)
        if mag > _max:
            _max = mag
print(_max)

