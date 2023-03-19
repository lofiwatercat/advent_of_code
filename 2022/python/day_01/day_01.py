
with open('input.txt') as file:
    data = [i for i in file.read().strip().split("\n")]


print(data)