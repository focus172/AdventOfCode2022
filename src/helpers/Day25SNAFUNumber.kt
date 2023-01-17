package helpers

import java.lang.Exception

class Day25SNAFUNumber {

    var representation: String
    var value: Long
    private val valueMap = generateMap()


    constructor(numberRepresentation: String) {
        representation = numberRepresentation
        value = getValue(numberRepresentation)
    }

    constructor(numberValue: Long) {
        representation = getRepresentation(numberValue)
        value = numberValue
    }

    private fun generateMap() : HashMap<Char, Long> {
        val retMap : HashMap<Char, Long> = HashMap()
        retMap['='] = -2
        retMap['-'] = -1
        retMap['0'] = 0
        retMap['1'] = 1
        retMap['2'] = 2
        return retMap
    }

    private fun getValue(stringRep: String) : Long {

        var returnedValue: Long = 0
        var index = 0
        val len: Int = stringRep.length

        while (index < len) {
            val currentLetter = stringRep[index]
            val characterValue : Long = valueMap[currentLetter] ?: throw Exception()
            returnedValue = returnedValue * 5 + characterValue
            index++
        }
        return returnedValue
    }

    private fun getRepresentation(value: Long) : String {
        var valueCopy = value
        val radix = 5;

        var buffer = CharArray(33)
        var index = 33;

        var shouldOverflow = false

        do {
            // get the value modulo 5
            val longZero: Long = 0

            var valueToAdd = 'a' // this allows rollovers of last digit to work
            if (valueCopy != longZero) {
                valueToAdd = (valueCopy % radix).toString()[0]
            } else {
                valueToAdd = '1'
                shouldOverflow = false
            }

            // add 1 if a flag is raised
            if (shouldOverflow) { valueToAdd++ }
            // if 3 or 4 OR 0 and shouldOverflow, raise a flag
            shouldOverflow = (valueToAdd == '3' || valueToAdd == '4' || valueToAdd == '5')
            // convert 3 and 4 to = and -
            if (valueToAdd == '3') { valueToAdd = '='}
            if (valueToAdd == '4') { valueToAdd = '-'}
            if (valueToAdd == '5') { valueToAdd = '0'}
            //add to array
            buffer[--index] = valueToAdd
            // "bit shift"
            valueCopy /= radix;


        }
        while (valueCopy > 0 || shouldOverflow)

        return String(buffer, index, 33 - index)
    }
}