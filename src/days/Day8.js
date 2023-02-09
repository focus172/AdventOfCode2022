
var raw = fileToString()
var dataArray = toArray(raw)
console.log("Problem 1: " + problem1(dataArray));
console.log("Problem 2: " + problem2(dataArray));

function problem1(dataLines) {

    var positionMap = new Map();

    // from the left
    for (let r = 0; r < dataLines.length; r++) {
        var maxFound = -1;
        let line = dataLines[r];
        for (let c = 0; c < line.length; c++) {
            let value = line[c];
            if (value > maxFound) {
                //console.log("adding " + r+", "+c + " because " + value + " > " + maxFound)
                positionMap.set(r+", "+c, true)
                maxFound = value
            }
        }
    }

    // from the right
    for (let r = 0; r < dataLines.length; r++) {
        var maxFound= -1;
        let line = dataLines[r];
        for (let c = line.length-1; c >= 0 ; c--) {
            let value = line[c];
            if (value > maxFound) {
                //console.log("adding " + r+", "+c + " because " + value + " > " + maxFound)
                positionMap.set(r+", "+c, true)
                maxFound = value
            }
        }
    }

    // from the top
    for (let c = 0; c < dataLines[0].length; c++) {
        var maxFound = -1;
        for (let r = 0; r < dataLines.length; r++) {
            let line = dataLines[r];
            let value = line[c];
            if (value > maxFound) {
                //console.log("adding " + r+", "+c + " because " + value + " > " + maxFound)
                positionMap.set(r+", "+c, true)
                maxFound = value
            }
        }
    }

    for (let c = 0; c < dataLines[0].length; c++) {
        var maxFound = -1;
        for (let r = dataLines.length-1; r >= 0 ; r--) {
            let line = dataLines[r];
            let value = line[c];
            if (value > maxFound) {
                //console.log("adding " + r+", "+c + " because " + value + " > " + maxFound)
                positionMap.set(r+", "+c, true)
                maxFound = value
            }
        }
    }

    return positionMap.size;
}

function problem2(dataLines) {
    var maxFound = 0;

    for (let r = 0; r < dataLines.length; r++) {
        var line = dataLines[r]
        for (let c = 0; c < line.length; c++) {
            var value = line[c]

            var fromTop = getFromTop(r, c, dataLines, value)
            var fromBottom = getFromBottom(r, c, dataLines, value)
            var fromLeft = getFromLeft(r, c, dataLines, value)
            var fromRight = getFromRight(r, c, dataLines, value)

            var score = fromBottom * fromLeft * fromTop * fromRight;

            if (score > maxFound) {
                console.log(" >> x: "+ c + ", y: "+ r)
                console.log("fromTop: " + fromTop + "\nfromBottom: " + fromBottom + "\nfromLeft: " + fromLeft + "\nfromRight: " + fromRight + "\n > score: " + score);
                maxFound = score
                console.log(maxFound)
            }
        }
    }

    return maxFound;
}

function getFromTop(row, col, dataArray, value) {
    var distance = 1
    var loop = true
    while (loop) {
        if (row - distance < 0) { loop = false; distance -= 1;}
        else if (dataArray[row-distance][col] >= value) { loop = false; }
        else { distance += 1; }
    }
    return distance
}

function getFromBottom(row, col, dataArray, value) {
    var distance = 1
    var loop = true
    while (loop) {
        if (row + distance >= dataArray.length) { loop = false; distance -= 1;}
        else if (dataArray[row+distance][col] >= value) { loop = false; }
        else { distance += 1; }
    }
    return distance
}

function getFromRight(row, col, dataArray, value) {
    var distance = 1
    var loop = true
    while (loop) {
        if (col + distance >= dataArray[row].length) { loop = false; distance -= 1;}
        else if (dataArray[row][col+distance] >= value) { loop = false; }
        else { distance += 1; }
    }
    return distance
}

function getFromLeft(row, col, dataArray, value) {
    var distance = 1
    var loop = true
    while (loop) {
        if (col - distance < 0) { loop = false; distance -= 1; }
        else if (dataArray[row][col-distance] >= value) { loop = false; }
        else { distance += 1; }
    }
    return distance
}

function toArray(rawData) {
    var lines = rawData.split("\n")
    var array = [];
    lines.forEach( line => array.push(line.split("")) )
    return array
}

function fileToString() {
    var fs = require("fs")
    var text = fs.readFileSync("/Users/evanstokdyk/code/AdventOfCode2022/src/inputs/Day8Input.txt").toString('utf-8')
    return text
}

