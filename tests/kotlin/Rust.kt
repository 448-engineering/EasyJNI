/** This file is used as a namespace for all the exported Rust functions. */
@file:JvmName("RustLibrary")

external fun nativeAssertions()
external fun rustyClass(): RustyClass

external fun rustyArray(): Array<String>
external fun rustyArrayInts(): Array<Int>

class RustyClass {
    private val message = "FROM_KOTLIN"

    fun getMessage(): String {
        return message
    }
}
