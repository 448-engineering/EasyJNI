fun main() {
    System.loadLibrary("rust_kotlin")

    nativeAssertions()

   
    val bar = rustyClass()
    val out = bar.getMessage()

    assert(out == "RUSTY_JNI_CLASS")
   


    
    val myArray = rustyArray()

    val testArray  = arrayOf("ONE", "TWO", "THREE");

    for ((index, value) in myArray.withIndex()) {

        val valueAtIndex = testArray[index]

        if (value != valueAtIndex) {
            throw RuntimeException("\nSTRING MISMATCH\nLEFT: $value, \nRIGHT:$valueAtIndex")
        }
    }

    val myInts = rustyArrayInts()

    val testIntsArray  = arrayOf(0,1,2);

    assert(myInts.contentEquals(testIntsArray))

    for ((index, value) in myInts.withIndex()) {

        val valueAtIndex = testIntsArray[index]

        if (value != valueAtIndex) {
            throw RuntimeException("\nINT MISMATCH\nLEFT: $value, \nRIGHT:$valueAtIndex")
        }

    }

    sillyDebugger()

    val myresult  = resultOfArrayString()

    if (myresult.failureData.isNotEmpty()) {        
        throw RuntimeException("`failureData` field is supposed to be empty for test to succeed")
    }

    for ((index, value) in myresult.successData.withIndex()) {

        val valueAtIndex = testArray[index]

        if (value != valueAtIndex) {
            throw RuntimeException("\nString MISMATCH\nLEFT: $value, \nRIGHT:$valueAtIndex")
        }

    }

    val redDirOutcome = dirReaderWithResult()    


    if (redDirOutcome.failureData.isNotEmpty()) {        
        throw RuntimeException("`failureData` field is supposed to be empty for test to succeed")
    }

    for (value in redDirOutcome.successData) {

        print(value)

    }
    
}