package org.nativescript.wcg.core

import dalvik.annotation.optimization.FastNative

class Base64 {
  companion object {
    init {
      GC.loadLib()
    }

    @JvmStatic
    fun atob(value: String): String {
      return nativeAtob(value)
    }

    @JvmStatic
    fun btoa(value: String): String {
      return nativeBtoa(value)
    }

    @FastNative
    @JvmStatic
    external fun nativeAtob(value: String): String

    @FastNative
    @JvmStatic
    external fun nativeBtoa(value: String): String
  }
}
