/** This file is used as a namespace for all the exported Rust functions. */
@file:JvmName("RustLibrary")

external fun nativeAssertions()
external fun rustyClass(): RustyClass

external fun rustyArray(): Array<String>
external fun rustyArrayInts(): Array<Int>
external fun resultOfArrayString(): ResultOfArrayString

external fun sillyDebugger(): String


class RustyClass {
    private val message = "FROM_KOTLIN"

    fun getMessage(): String {
        return message
    }
}


class ResultOfArrayString {
    val successData: Array<String> = emptyArray()
    val failureData: Array<String> = emptyArray()
}