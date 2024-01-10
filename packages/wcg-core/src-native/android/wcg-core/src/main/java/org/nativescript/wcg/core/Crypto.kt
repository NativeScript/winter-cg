package org.nativescript.wcg.core

import dalvik.annotation.optimization.FastNative
import java.nio.Buffer
import java.nio.ByteBuffer
import java.nio.FloatBuffer
import java.nio.IntBuffer
import java.nio.LongBuffer
import java.nio.ShortBuffer

class Crypto {
  companion object {
    init {
      GC.loadLib()
    }

    @JvmStatic
    fun randomUUID(): String {
      return nativeRandomUUID()
    }

    @JvmStatic
    fun getRandomValues(value: ByteArray) {
      return nativeGetRandomValues(value)
    }

    @JvmStatic
    fun getRandomValues(value: ShortArray) {
      return nativeGetRandomValuesShort(value)
    }

    @JvmStatic
    fun getRandomValues(value: IntArray) {
      return nativeGetRandomValuesInt(value)
    }

    @JvmStatic
    fun getRandomValues(value: LongArray) {
      return nativeGetRandomValuesLong(value)
    }

    @JvmStatic
    fun getRandomValues(value: ByteBuffer) {
      if (value.isDirect) {
        return nativeGetRandomValuesBuffer(value)
      }
      return nativeGetRandomValues(value.array())
    }

    @JvmStatic
    fun getRandomValues(value: ShortBuffer) {
      if (value.isDirect) {
        return nativeGetRandomValuesBufferShort(value)
      }
      return nativeGetRandomValuesShort(value.array())
    }

    @JvmStatic
    fun getRandomValues(value: IntBuffer) {
      if (value.isDirect) {
        return nativeGetRandomValuesBufferInt(value)
      }
      return nativeGetRandomValuesInt(value.array())
    }

    @JvmStatic
    fun getRandomValues(value: LongBuffer) {
      if (value.isDirect) {
        return nativeGetRandomValuesBufferLong(value)
      }
      return nativeGetRandomValuesLong(value.array())
    }


    @JvmStatic
    fun getRandomValuesByte(value: ByteBuffer) {
      return getRandomValuesByte(value)
    }

    @JvmStatic
    fun getRandomValuesShort(value: ShortBuffer) {
      return getRandomValues(value)
    }

    @JvmStatic
    fun getRandomValuesInt(value: IntBuffer) {
      return getRandomValues(value)
    }

    @JvmStatic
    fun getRandomValuesLong(value: LongBuffer) {
      return getRandomValues(value)
    }

    @FastNative
    @JvmStatic
    private external fun nativeRandomUUID(): String

    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValues(value: ByteArray)

    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesBuffer(value: ByteBuffer)

    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesShort(value: ShortArray)


    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesInt(value: IntArray)


    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesLong(value: LongArray)


    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesBufferShort(value: Any)

    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesBufferInt(value: Any)

    @FastNative
    @JvmStatic
    private external fun nativeGetRandomValuesBufferLong(value: Any)


  }
}
