def readFile(lines):
    activeLines = []
    time = 0

    count = 0
    for line in lines:
        if count == 0:
            time = int(line.strip())
        else:
            busses = line.strip().split(',')
            for bus in busses:
                try:
                    busNum = int(bus)
                    activeLines.append(busNum)
                except Exception as e:
                    activeLines.append(-1);
        count+=1

    return time,activeLines


def part1(startTime, busLines):
    activeLines = busLines
    time = startTime
    timeUntil = []

    for bus in activeLines:
        if bus > 0:
            timeUntil.append([bus-(time%bus), bus])

    timeUntil.sort(key=lambda tup: tup[0])

    return timeUntil[0]

def part2(busLines):
    timeUntil = []

    offset = 0
    for bus in busLines:
        if bus > 0:
            timeUntil.append([bus, offset, offset%bus])
        offset += 1

    it = timeUntil[0][0]
    curTime = it
    for b,i,o in timeUntil[1:]:
        while True:
            if (curTime + i) % b == 0:
                break;
            curTime += it
        it *= b

    return curTime

if __name__ == '__main__':
    inputFile = open("input.txt", "r")
    inputFileLines = inputFile.readlines()

    startTime,busLines = readFile(inputFileLines)

    waitTime = part1(startTime, busLines)
    print("Bus:",waitTime[1],
          "Time:",waitTime[0],
          "Answer:", waitTime[0]*waitTime[1])

    time = part2(busLines)
    print("Time:",time)

    inputFile.close()
