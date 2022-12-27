
var raw = fileToString()
var dataArray = toArray(raw)
console.log("Problem 1: " + problem1(dataArray));
console.log("Problem 2: " + problem2());

function problem1(dataLines) {
    var helpedArray = []
    dataLines.forEach( line => function(line) {
        var lineArray = []
        line.forEach( letter => function(letter) {
            let helperObj = {
                value: letter,
                fromTop: false,
                fromBottom: false,
                fromLeft: false,
                fromRight: false
            };
            lineArray.push(helperObj)
        })
        helpedArray.push(lineArray)
    })
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
    var text = fs.readFileSync("./src/inputs/Day8Input.txt").toString('utf-8')
    return text
}