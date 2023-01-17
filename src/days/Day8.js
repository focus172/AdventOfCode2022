
var raw = fileToString()
var dataArray = toArray(raw)
console.log("Problem 1: " + problem1(dataArray));
console.log("Problem 2: " + problem2());

function problem1(dataLines) {
    //var helpedArray = []
    dataLines.forEach( line => function(line) {
        //var lineArray = []
        line.forEach( letter => function(letter) {
            let helperObj = {
                value: letter,
                fromTop: false,
                fromBottom: false,
                fromLeft: false,
                fromRight: false
            };
            letter.child(helperObj);
            console.log(letter)
        });
        //helpedArray.push(lineArray);
    });

    var maxFoundFromLeft = 0;

    /*
    for (let i = 0; i < dataLines.length; i++) {
        if dataLines[i] > maxFoundFromLeft {
            maxFoundFromLeft = dataLines[i]
        } else {
            helpedArray[i][]
        }
    }

    */

    return 0;

}

function problem2() {
    return 0;
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