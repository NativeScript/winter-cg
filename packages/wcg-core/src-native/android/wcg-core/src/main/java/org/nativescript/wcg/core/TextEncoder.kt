package org.nativescript.wcg.core

import dalvik.annotation.optimization.CriticalNative
import dalvik.annotation.optimization.FastNative
import java.nio.ByteBuffer


class TextEncoder @JvmOverloads constructor(encoding: String = "utf-8") {
  private var native: Long

  init {
    native = nativeCreate(encoding)
    if (native == 0L) {
      throw Error("Invalid Encoding")
    }
  }

  fun encode(value: String): ByteArray {
    return nativeEncode(native, value)
  }

  fun encodeBuffer(value: String): ByteBuffer {
    return nativeEncodeBuffer(native, value)
  }

  @Synchronized
  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      nativeDestroy(native)
      native = 0L
    }
  }


  companion object {

    init {
      GC.loadLib()
    }

    @FastNative
    @JvmStatic
    private external fun nativeCreate(encoding: String): Long

    @FastNative
    @JvmStatic
    private external fun nativeGetEncoding(encoder: Long): String

    @FastNative
    @JvmStatic
    private external fun nativeEncode(encoder: Long, value: String): ByteArray

    @FastNative
    @JvmStatic
    private external fun nativeEncodeBuffer(encoder: Long, value: String): ByteBuffer

    @CriticalNative
    @JvmStatic
    private external fun nativeDestroy(encoder: Long)
  }
}
